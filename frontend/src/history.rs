use yew::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlInputElement;

// temporary
use rand::prelude::*;

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

#[function_component(GamesList)]
fn games_list(GameProps { games }: &GameProps) -> Html {
    games.iter().map(|game| html! {
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
            <td>{{game.player_1_name.clone()}}</td>
            <td>{{game.player_2_name.clone()}}</td>
            <td>{{match game.player1_won {
                1 => {
                    game.player_1_name.clone()
                },
                2 => {
                    game.player_2_name.clone()
                },
                _ => {
                    "draw".to_string()
                }
            }}}</td>
            <td>{game.date}</td>
        </tr>
    }).collect()
}

#[function_component(History)]
pub fn history() -> Html {
    let games_list = use_state(|| Vec::<Game>::new());
    let p1name = use_state(|| "".to_string());
    let p2name = use_state(|| "".to_string());

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

    let updatep1name = {
        let p1name = p1name.clone();
        Callback::from(move |name_event: InputEvent| {
            // DOM query from: https://stackoverflow.com/questions/71690906/how-to-query-and-update-the-dom-with-yew
            let event: Event = name_event.dyn_into().unwrap_throw();
            let input_elem: HtmlInputElement = event.target().unwrap_throw().dyn_into().unwrap_throw();
            let value = input_elem.value();
            p1name.set(value);
        })
    };
    let updatep2name = {
        let p2name = p2name.clone();
        Callback::from(move |name_event: InputEvent| {
            // DOM query from: https://stackoverflow.com/questions/71690906/how-to-query-and-update-the-dom-with-yew
            let event: Event = name_event.dyn_into().unwrap_throw();
            let input_elem: HtmlInputElement = event.target().unwrap_throw().dyn_into().unwrap_throw();
            let value = input_elem.value();
            p2name.set(value);
        })
    };

    html! {
        <div id="main" ng-controller="ScoreBoardCtrl">

            <div class="body-container" id="services">
                <div class="main-header">
                    <b>{"Game History"}</b>
                </div>
                <hr class="header-divider"/>
                
                <div class="bottom-table">
                    <table>
                        <tr>
                            <th>{"Game-ID"}</th>
                            <th>{"Game Type"}</th>
                            <th>{"Player1"}</th>
                            <th>{"Player2"}</th>
                            <th>{"Winner"}</th>
                            <th>{"When Played"}</th>
                        </tr>
                        <GamesList games={(*games_list).clone()}/>
                    </table>      
                </div>

            </div>

        </div>
    }
}