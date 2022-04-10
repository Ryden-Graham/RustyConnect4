use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
            <div class="body-container" id="services">
                <div class="main-header">
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