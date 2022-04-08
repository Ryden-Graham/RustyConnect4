use yew::prelude::*;

#[function_component(Connect4Computer)]
pub fn connect4Computer() -> Html {
    html! {
        <div class="body-container" id="services">
            <div class="main-header">
                <b>{"Enter Your Name"}</b>
            </div>
            <hr class="header-divider"/>
            <div class="name-entry-container">
                <input
                    class="name-textbox"
                    type="text"
                    placeholder="Your Name"
                />
                <button
                    class="button start-game"
                    type="button"
                >
                {"Start Game"}
                </button>
                <br />
            </div>
        </div>
    }
}