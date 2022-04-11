use yew::prelude::*;
use crate::canvas_toot::CanvasTOOTComputer;

#[function_component(TootOttoComputer)]
pub fn connect4Computer() -> Html {
    html! {
        <div class="body-container" id="services">
            <div class="main-header">
                <b>{"Enter Your Name"}</b>
            </div>
            <hr class="header-divider"/>
            <CanvasTOOTComputer />
        </div>
    }
}