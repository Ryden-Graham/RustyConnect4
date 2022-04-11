use yew::prelude::*;
use crate::canvas_tootHuman::CanvasTOOTHuman;

#[function_component(TootOttoHuman)]
pub fn connect4Computer() -> Html {
    html! {
        <div class="body-container" id="services">
            <div class="main-header">
                <b>{"Enter Your Name"}</b>
            </div>
            <hr class="header-divider"/>
            <CanvasTOOTHuman />
        </div>
    }
}