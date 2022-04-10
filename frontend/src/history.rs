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
    game_type: bool, 
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
        // <p>{format!("id: {}, type: {}, p1: {}, p2: {}, p2isCPU: {}, player1_won: {}, date: {}",
        //     game.id,
        //     game.game_type,
        //     game.player_1_name,
        //     game.player_2_name,
        //     game.player_2_is_computer,
        //     game.player1_won,
        //     game.date
        // )}</p>
        <tr>
            <td>{{game.id}}</td>
            <td>{{game.game_type}}</td>
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
    // let value = use_state(|| 0);

    // let onclickadd = {
    //     let value = value.clone();
    //     Callback::from(move |_| value.set(*value + 1))
    // };
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
        // let p1name = p1name.clone();
        // let p2name = p2name.clone();
        Callback::from(move |_| {
            let mut rng = rand::thread_rng();
            let new_game = Game {
                id: 0,
                game_type: rng.gen::<f32>() < 0.5, 
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
            // <div class="w3-container" id="services" style="margin-top:75px">
            //     <h5 class="w3-xxxlarge w3-text-red"><b>{"How to Play Connect 4"}</b></h5>
            //     <hr style="width:50px;border:5px solid red" class="w3-round"/>
            //     <p>
            //         {"HISTORY PAGE"}
            //     </p>
            //     <br/>
            //     <div><h5>{"To play Connect 4 follow the following steps:"}</h5></div>
            //     <ul>

            //         <li>{"A new game describes discs of which color belongs to which player"}</li>

            //         <li>{"Click on the desired column on the game board to place your disc"}</li>

            //         <li>{"Try to connect 4 of your colored discs either horizontally or vertically or diagonally"}</li>

            //     </ul>
            //     <br/>
            //     <p>
            //         {"For More information on Connect 4 click "}<a href="https://en.wikipedia.org/wiki/Connect_Four">{"here"}</a>
            //     </p>
            // </div>
            <div id="main" ng-controller="ScoreBoardCtrl">

                <div class="body-container" id="services" style="margin-top:75px">
                    <h5 class="main-header"><b>
                        {"Game History"}
                    </b></h5>
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
                    <hr style="width:50px;border:5px solid red" class="w3-round"/>
                    
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