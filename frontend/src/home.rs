use yew::prelude::*;
// use reqwasm::http::Request;
// use reqwest::*;
use serde::{Serialize, Deserialize};

// use hyper::header::{Headers, AccessControlAllowOrigin};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct User {
    name: String,
    wins: u32,
    losses: u32
}

#[derive(Properties, PartialEq)]
struct UserProps {
    users: Vec<User>,
}

#[function_component(UsersList)]
fn users_list(UserProps { users }: &UserProps) -> Html {
    users.iter().map(|user| html! {
        <p>{format!("{}: {}, {}", user.name, user.wins, user.losses)}</p>
    }).collect()
}

#[function_component(Home)]
pub fn home() -> Html {
    let data = use_state(|| vec![]);
    {
        let data = data.clone();
        use_effect_with_deps(move |_| {
            let data = data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_data: Vec<User> = reqwest::Client::new()
                    .get("http://127.0.0.1:7000")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                data.set(fetched_data);
            });
            || ()
        }, ());
    }
    
    let debug = use_state(|| "0".to_string());
    let send_data = {
        let data = data.clone();
        Callback::from(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let test_user = User {
                    name: "mr_NODED_abuser".to_string(),
                    wins: 100,
                    losses: 10
                };
            
                let sent = reqwest::Client::new()
                    .post("http://127.0.0.1:7000/client")
                    .json(&test_user)
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
                    <UsersList users={(*data).clone()}/>
                    <button onclick={send_data}>{ "Send Data!!!!" }</button>
                    <p>{debug.to_string()}</p>
                    <b>{"How to Play Connect 4"}</b>
                </div>
                <hr class="header-divider"/>
                <p>
                    {"Connect Four is a two-player connection game in which the players take turns dropping colored discs from the top into a seven-column, six-row vertically suspended grid. The objective of the game is to be the first to form a horizontal, vertical, or diagonal line of four of one's own discs."}
                </p>
                <br/>
                <p class="sub-header">
                    {"To play Connect 4 follow the following steps:"}
                </p>
                <ul>
                    <li>{"A new game describes discs of which color belongs to which player"}</li>
                    <li>{"Click on the desired column on the game board to place your disc"}</li>
                    <li>{"Try to connect 4 of your colored discs either horizontally or vertically or diagonally"}</li>
                </ul>
                <br/>
                <p>
                    {"For More information on Connect 4 click "}
                    <a href="https://en.wikipedia.org/wiki/Connect_Four">{"here"}</a>
                    {"."}
                </p>
            </div>
        }
}