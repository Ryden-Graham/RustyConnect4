use yew::prelude::*;
use crate::canvas::CanvasModel;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}
use Difficulty::*;

#[function_component(Connect4Computer)]
pub fn connect4Computer() -> Html {

    let player_name = use_state(|| "".to_string());
    let display_state = use_state(|| "".to_string());
    let is_game_on = use_state(|| false);
    let disabled = use_state(|| false);
    let start_game = {
        is_game_on.set(true);
        disabled.set(true);
        display_state.set("block".to_string());
    };
    let end_game = {
        is_game_on.set(false);
        disabled.set(false);
        display_state.set("none".to_string());
    };

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
                    onclick={start_game}
                    disabled={*disabled}
                >
                {"Start Game"}
                </button>
                <br />
            </div>
            <div style={format!("display: {}", *display_state)}>
                <br/>
                <h4>{format!("New Game: {} Vs Computer", *player_name)}</h4>
                <small>{format!("(Disc Colors: {} - ", *player_name)} <b>{"Red"}</b> {"   and    Computer - "} <b>{"Yellow)"}</b></small>
                <br/>
                <CanvasModel  
                    canvas_id = "connect_computer" 
                    player1 = {*player_name.clone()}
                    player2 = "Computer" 
                    // difficulty = self.difficulty,
                    game_done_cbk={end_game}/>
            </div>
        </div>
    }
}