use yew::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use std::iter::FromIterator;
use itertools::Itertools;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct Game {
    id: u32,
    game_type_is_c4: bool, 
    player_1_name: String,
    player_2_name: String,
    player_2_is_computer: bool,
    player1_won: u32,
    date: DateTime<Utc>
}

#[derive(Properties, PartialEq)]
struct GameProps {
    games: Vec<Game>,
}

#[function_component(PlayerList)]
fn players_list(GameProps { games }: &GameProps) -> Html {
    let mut names: Vec<String> = games
        .iter()
        .flat_map(|game| match game.player_2_is_computer {
            false => {
                [game.player_1_name.clone(), game.player_2_name.clone()].into_iter()
            },
            true => {
                [game.player_1_name.clone(), game.player_1_name.clone()].into_iter()
            }
        })
        .collect();
    names.sort();
    names
        .iter()
        .unique().map(|player| {
        let wins = games
            .iter()
            .filter(|game| ((&game.player_1_name == player) && (game.player1_won == 1)) || ((&game.player_2_name == player) && (game.player1_won == 2)))
            .count();
        let losses = games
            .iter()
            .filter(|game| ((&game.player_1_name == player) && (game.player1_won == 2)) || ((&game.player_2_name == player) && (game.player1_won == 1)))
            .count();
        let draws = games
            .iter()
            .filter(|game| ((&game.player_1_name == player) && (game.player1_won == 0)) || ((&game.player_2_name == player) && (game.player1_won == 0)))
            .count();
        let percentage = ((wins as f64) * 100000.0/(wins as f64 + losses as f64 + draws as f64)).round()/1000.0;
        html! {
            <tr>
                <td>{{player}}</td>
                <td>{{wins}}</td>
                <td>{{losses}}</td>
                <td>{{draws}}</td>
                <td>{{percentage}}</td>
            </tr>
        }
    }).collect()
}

#[function_component(ComputerList)]
fn computers_list(GameProps { games }: &GameProps) -> Html {
    games.iter().filter(|game| (game.player_2_is_computer == true) && (game.player1_won == 2)).map(|game| {
        html! {
        <tr>
            <td>{{game.id}}</td>
            <td>{{match game.game_type_is_c4 {
                true => {
                    "Connect-4".to_string()
                },
                false => {
                    "Toot-Otto".to_string()
                }
                }}}</td>
            <td>{{game.player_2_name.clone()}}</td>
            <td>{{game.player_1_name.clone()}}</td>
            <td>{game.date}</td>
        </tr>
    }}).collect()
}

#[function_component(Scores)]
pub fn history() -> Html {
    let games_list = use_state(|| Vec::<Game>::new());

    {
        let games_list = games_list.clone();
        use_effect_with_deps(move |_| {
            let games_list = games_list.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_data: Vec<Game> = reqwest::Client::new()
                    .get("http://127.0.0.1:7000")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                games_list.set(fetched_data);
            });
            || ()
        }, ());
    }

    let total_games = (*games_list)
        .clone()
        .iter()
        .count();
    let games_against_cpu = (*games_list)
        .clone()
        .iter()
        .filter(|game| game.player_2_is_computer == true)
        .count();
    let games_cpu_won = (*games_list)
        .clone()
        .iter()
        .filter(|game| (game.player_2_is_computer == true) && (game.player1_won == 2))
        .count();

    html! {
            <div id="main" ng-controller="ScoreBoardCtrl">

                <div class="body-container" id="services">
                    <div class="main-header">
                        <b>{"Score Board"}</b>
                    </div>
                    <hr class="header-divider"/>
                    <p class="sub-header">
                        {"Games Won by Computer"}
                    </p>
                    <div>
                        <table>
                            <tr>
                                <th>{"Total Games Played"}</th>
                                <th>{"Games Against Computer"}</th>
                                <th>{"Games Computer Won"}</th>
                            </tr>
                            <tr>
                                <th>{total_games}</th>
                                <th>{games_against_cpu}</th>
                                <th>{games_cpu_won}</th>
                            </tr>
                        </table>      
                    </div>
                    <br/>
                    <p class="sub-header">
                        {"Details of Games Won by Computer"}
                    </p>
                    <div>
                        <table>
                            <tr>
                                <th>{"Game-ID"}</th>
                                <th>{"Game Type"}</th>
                                <th>{"Winner"}</th>
                                <th>{"Played Against"}</th>
                                <th>{"When Played"}</th>
                            </tr>
                            <ComputerList games={(*games_list).clone()}/>
                        </table>      
                    </div>
                    <br/>
                    <p class="sub-header">
                        {"Details of Games Won by All Players"}
                    </p>
                    <div class="bottom-table">
                        <table>
                            <tr>
                                <th>{"Player"}</th>
                                <th>{"Wins"}</th>
                                <th>{"Losses"}</th>
                                <th>{"Draws"}</th>
                                <th>{"Win Percentage"}</th>
                            </tr>
                            <PlayerList games={(*games_list).clone()}/>
                        </table>      
                    </div>
                </div>

            </div>
        }
}