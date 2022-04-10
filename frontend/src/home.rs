use yew::prelude::*;
use chrono::{DateTime, Utc};
// use reqwasm::http::Request;
// use reqwest::*;
use serde::{Serialize, Deserialize};
use chrono::prelude::*;

// use hyper::header::{Headers, AccessControlAllowOrigin};

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
        <p>{format!("id: {}, type: {}, p1: {}, p2: {}, p2isCPU: {}, player1_won: {}, date: {}",
            game.id,
            game.game_type,
            game.player_1_name,
            game.player_2_name,
            game.player_2_is_computer,
            game.player1_won,
            game.date
        )}</p>
    }).collect()
}

#[function_component(Home)]
pub fn home() -> Html {
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
    
    let send_data = {
        let num_games = games_list.len();
        Callback::from(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let utc: DateTime<Utc> = Utc::now();

                let new_game = Game {
                    id: num_games as u32 + 1,
                    game_type: true, 
                    player_1_name: "mr_NODED_abuser".to_string(),
                    player_2_name: "computer".to_string(),
                    player_2_is_computer: true,
                    player1_won: false,
                    date: utc
                };
            
                let sent = reqwest::Client::new()
                    .post("http://127.0.0.1:7000/client")
                    .json(&new_game)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await.unwrap();
            });
        })
    };

    let nuke = {
        Callback::from(move |_| {
            wasm_bindgen_futures::spawn_local(async move {          
                let sent = reqwest::Client::new()
                    .post("http://127.0.0.1:7000/command")
                    .body("nuke the world")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await.unwrap();
            });
        })
    };

    html! {
            <div class="body-container" id="services">
                <div class="main-header">
                    <GamesList games={(*games_list).clone()}/>
                    <button onclick={send_data}>{ "Add Game!!!" }</button>
                    <button onclick={nuke}>{ "NUKE DATABASE" }</button>
                    <b>{"Welcome"}</b>
                </div>
                <hr class="header-divider"/>
                <p>
                    {"This application contains the following two board games, both in human Vs. human and human Vs. Computer versions."}
                </p>
                <ul>
                    <li>{"A new game describes discs of which color belongs to which player"}</li>
                    <li>{"Click on the desired column on the game board to place your disc"}</li>
                    <li>{"Try to connect 4 of your colored discs either horizontally or vertically or diagonally"}</li>
                </ul>
                <p>
                    {"Select the game of your choice from the side bar, and start playing. Enjoy!"}
                </p>
            </div>
        }
}