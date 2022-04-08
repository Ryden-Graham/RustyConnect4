use yew::prelude::*;

#[function_component(HowToToot)]
pub fn howToToot() -> Html {
    html! {
        <div class="body-container" id="services">
            <div class="main-header">
                <b>{"How to Play TOOT-OTTO"}</b>
            </div>
            <hr class="header-divider"/>
            <p>
                {"TOOT-OTTO is a fun strategy game for older players who like tic-tac-toe and checkers. One player is TOOT and the other player is OTTO. Both players can place both T's and O's, based on their choice. The first player who spells his or her winning combination - horizontally, vertically or diagonally - wins!"}
            </p>
            <br/>
            <p class="sub-header">
                {"To play TOOT-OTTO follow the following steps:"}
            </p>
            <ul>
                <li>{"A new game describes which player is TOOT and which is OTTO"}</li>
                <li>{"Select the disc type T or O that you want to place"}</li>
                <li>{"Click on the desired column on the game board to place your disc"}</li>
                <li>{"Try to spell TOOT or OTTO based on your winning combination, either horizontally or vertically or diagonally"}</li>
            </ul>
            <br/>
            <p>
                {"For More information on TOOT-OTTO click "}
                <a href="https://boardgamegeek.com/boardgame/19530/toot-and-otto">{"here"}</a>
                {"."}
            </p>
        </div>
        }
}