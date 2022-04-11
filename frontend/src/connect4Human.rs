use yew::prelude::*;
use crate::canvasHuman::CanvasHuman;

#[function_component(Connect4Human)]
pub fn connect4Human() -> Html {
    html! {
        <div class="body-container" id="services">
            <div class="main-header">
                <b>{"Enter Your Name"}</b>
            </div>
            <hr class="header-divider"/>
            <CanvasHuman />
        </div>
    }
}