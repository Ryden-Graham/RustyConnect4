use yew::prelude::*;
use crate::canvas::CanvasModel;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}
// use Difficulty::*;

#[function_component(Connect4Computer)]
pub fn connect4Computer() -> Html {
    // let end_game = {
    //     is_game_on.set(false);
    //     disabled.set(false);
    //     display_state.set("none".to_string());
    // };

    html! {
        <div class="body-container" id="services">
            <div class="main-header">
                <b>{"Enter Your Name"}</b>
            </div>
            <hr class="header-divider"/>
            <CanvasModel />
        </div>
    }
}