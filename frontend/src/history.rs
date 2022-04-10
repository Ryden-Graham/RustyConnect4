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
    player1_won: bool,
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
                true => {
                    game.player_1_name.clone()
                },
                false => {
                    game.player_2_name.clone()
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
    
    let send_data = {
        let games_list = games_list.clone();
        let utc: DateTime<Utc> = Utc::now();
        Callback::from(move |_| {
            let mut rng = rand::thread_rng();
            let new_game = Game {
                id: 0,
                game_type_is_c4: rng.gen::<f32>() < 0.5, 
                player_1_name: p1name.to_string(),
                player_2_name: match p2name.clone().len() {
                    0 => {
                        "computer".to_string()
                    },
                    _ => {
                        p2name.to_string()
                    }
                },
                player_2_is_computer: p2name.len() == 0,
                player1_won: rng.gen::<f32>() < 0.5,
                date: utc
            };
            
            let games_list = games_list.clone();
            wasm_bindgen_futures::spawn_local(async move {
                reqwest::Client::new()
                    .post("http://127.0.0.1:7000/client")
                    .json(&new_game)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await.unwrap();

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
        })
    };

    let nuke = {
        let games_list = games_list.clone();
        Callback::from(move |_| {
            wasm_bindgen_futures::spawn_local(async move {          
                reqwest::Client::new()
                    .post("http://127.0.0.1:7000/command")
                    .body("nuke the world")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await.unwrap();
            });
            games_list.set(Vec::<Game>::new());
        })
    };

    html! {
        <div id="main" ng-controller="ScoreBoardCtrl">

            <div class="body-container" id="services" style="margin-top:75px">
                <div class="main-header">
                    <b>{"Game History"}</b>
                </div>
                <button class="button start-game" onclick={send_data}>{ "Add Game!!!" }</button>
                <input
                    class="name-textbox"
                    type="text"
                    placeholder="Player1 name"
                    oninput={updatep1name}
                />
                <input
                    class="name-textbox"
                    type="text"
                    placeholder="Player2 name"
                    oninput={updatep2name}
                />
                <button class="button start-game" onclick={nuke}>{ "NUKE DATABASE" }</button>
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