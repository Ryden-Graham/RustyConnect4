use yew::prelude::*;
use reqwasm::http::Request;

#[function_component(Home)]
pub fn home() -> Html {
    // let value = use_state(|| 0);

    // let onclickadd = {
    //     let value = value.clone();
    //     Callback::from(move |_| value.set(*value + 1))
    // };

    // let data = use_state(|| vec![]);
    // {
    //     let data = data.clone();
    //     use_effect_with_deps(move |_| {
    //         let data = data.clone();
    //         wasm_bindgen_futures::spawn_local(async move {
    //             let fetched_data: Vec<String> = Request::get("https://yew.rs/tutorial/data.json")
    //                 .send()
    //                 .await
    //                 .unwrap()
    //                 .json()
    //                 .await
    //                 .unwrap();
    //             data.set(fetched_data);
    //         });
    //         || ()
    //     }, ());
    // }

    html! {
            <div class="body-container" id="services">
                <div class="main-header">
                    // <b>{(*data).clone()}</b>
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