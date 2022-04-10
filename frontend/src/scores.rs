use yew::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use chrono::prelude::*;

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

#[function_component(Scores)]
pub fn history() -> Html {
    // let value = use_state(|| 0);

    // let onclickadd = {
    //     let value = value.clone();
    //     Callback::from(move |_| value.set(*value + 1))
    // };
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
    
    // let send_data = {
    //     let games_list = games_list.clone();
    //     let num_games = games_list.len();
    //     let utc: DateTime<Utc> = Utc::now();
    //     Callback::from(move |_| {
    //         let new_game = Game {
    //             id: num_games as u32 + 1,
    //             game_type: true, 
    //             player_1_name: "mr_NODED_abuser".to_string(),
    //             player_2_name: "computer".to_string(),
    //             player_2_is_computer: true,
    //             player1_won: false,
    //             date: utc
    //         };
    //         // games_list.push(new_game.clone());
    //         let mut new_list = (*games_list).clone();
    //         new_list.push(new_game.clone());
    //         wasm_bindgen_futures::spawn_local(async move {
    //             let sent = reqwest::Client::new()
    //                 .post("http://127.0.0.1:7000/client")
    //                 .json(&new_game)
    //                 .send()
    //                 .await
    //                 .unwrap()
    //                 .text()
    //                 .await.unwrap();
    //         });
    //         games_list.set(new_list.to_vec());
    //     })
    // };

    // let nuke = {
    //     let games_list = games_list.clone();
    //     Callback::from(move |_| {
    //         wasm_bindgen_futures::spawn_local(async move {          
    //             let sent = reqwest::Client::new()
    //                 .post("http://127.0.0.1:7000/command")
    //                 .body("nuke the world")
    //                 .send()
    //                 .await
    //                 .unwrap()
    //                 .text()
    //                 .await.unwrap();
    //         });
    //         games_list.set(Vec::<Game>::new());
    //     })
    // };

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
                        {"Score Board"}
                    </b></h5>
                    // <button class="button start-game" onclick={send_data}>{ "Add Game!!!" }</button>
                    // <button class="button start-game" onclick={nuke}>{ "NUKE DATABASE" }</button>
                    <hr style="width:50px;border:5px solid red" class="w3-round"/>
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
                            // <GamesList games={(*games_list).clone()}/>
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
                            // <GamesList games={(*games_list).clone()}/>
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
                                <th>{"Win Percentage"}</th>
                            </tr>
                            // <GamesList games={(*games_list).clone()}/>
                        </table>      
                    </div>
                </div>

            </div>
        }
}