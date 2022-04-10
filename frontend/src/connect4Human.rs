use yew::prelude::*;

#[function_component(Connect4Human)]
pub fn connect4Computer() -> Html {
    html! {
        <div class="body-container" id="services">
            <div class="main-header">
                <b>{"Enter Your Name"}</b>
            </div>
            <hr class="header-divider"/>
        </div>
    }
}