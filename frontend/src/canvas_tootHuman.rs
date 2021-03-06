use std::error::Error;
use std::ptr::null;
use stdweb::traits::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::Date;
use stdweb::web::FillRule;
use stdweb::web::{document, window, CanvasRenderingContext2d};
use stdweb::web::event::ClickEvent;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use web_sys::HtmlInputElement;
use yew::{prelude::*, virtual_dom::VNode, Properties};
use log;
use yew_hooks::use_is_mounted;
use crate::connect4Computer::Difficulty::{self, *};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct Game {
    id: u32,
    game_type_is_c4: bool, 
    player_1_name: String,
    player_2_name: String,
    player_2_is_computer: bool,
    player1_won: u32,
    date: DateTime<Utc>
}

#[inline(always)]
fn get_canvas_element() -> web_sys::HtmlCanvasElement {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}

#[inline(always)]
fn get_canvas_context() -> web_sys::CanvasRenderingContext2d {
    get_canvas_element().get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

#[function_component(CanvasTOOTHuman)]
pub fn canvasTOOTHuman() -> Html {
    // Boolean check state variables
    let is_mounted = use_is_mounted();
    let canvas_context_exists = use_state(|| false);
    let is_canvas_drawn = use_state(|| false);
    let is_game_on = use_state(|| false);
    let disabled = use_state(|| false);
    let game_won = use_state(|| false);
    let colorblind_enabled = use_state(|| false);
    let colorblind_enabled_display = use_state(|| false);
    
    // Complex state variables
    let canvas_context:UseStateHandle<Option<web_sys::CanvasRenderingContext2d>> = use_state(|| None);
    let canvas:UseStateHandle<Option<web_sys::HtmlCanvasElement>> = use_state(|| None);
    let player_name = use_state(|| "".to_string());
    let display_state = use_state(|| "".to_string());
    let game_map = use_state(|| vec![vec![0; 7]; 6]);
    let dummy_map = use_state(|| vec![vec![0; 7]; 6]);
    let player_name_1 = use_state(|| "".to_string());
    let player_name_2 = use_state(|| "".to_string());
    let player_name_1_display = use_state(|| "".to_string());
    let player_name_2_display = use_state(|| "".to_string());
    let pending_name_1 = use_state(|| "".to_string());
    let pending_name_2 = use_state(|| "".to_string());
    let current_turn = use_state(|| 0);

    let is_player_1_turn:usize = (*game_map).clone().iter().map(|column| column.iter().filter(|circle_number| **circle_number != 0).count()).sum::<usize>() % 2;
    
    let drop_disk_1 = {
        let game_map = game_map.clone();
        let dummy_map = dummy_map.clone();
        let current_turn = current_turn.clone();
        Callback::from(move |_| {
            let mut game_map_clone = (*game_map).clone();
            for i in 0..6 {
                if game_map_clone[5-i][0] == 0 {
                    game_map_clone[5-i][0] = match is_player_1_turn {
                        0 => {
                            1
                        },
                        _ => {
                            -1
                        }
                    };
                    current_turn.set((*current_turn) + 1);
                    break;
                }
            }
            game_map.set(game_map_clone);
            let mut dummy_map_clone = (*dummy_map).clone();
            for i in 0..6 {
                if dummy_map_clone[5-i][0] == 0 {
                    dummy_map_clone[5-i][0] = match is_player_1_turn {
                        0 => {
                            84
                        },
                        _ => {
                            79
                        }
                    };
                    break;
                }
            }
            dummy_map.set(dummy_map_clone);
        })
    };

    let drop_disk_2 = {
        let game_map = game_map.clone();
        let dummy_map = dummy_map.clone();
        let current_turn = current_turn.clone();
        Callback::from(move |_| {
            let mut game_map_clone = (*game_map).clone();
            for i in 0..6 {
                if game_map_clone[5-i][1] == 0 {
                    game_map_clone[5-i][1] = match is_player_1_turn {
                        0 => {
                            1
                        },
                        _ => {
                            -1
                        }
                    };
                    current_turn.set((*current_turn) + 1);
                    break;
                }
            }
            game_map.set(game_map_clone);
            let mut dummy_map_clone = (*dummy_map).clone();
            for i in 0..6 {
                if dummy_map_clone[5-i][1] == 0 {
                    dummy_map_clone[5-i][1] = match is_player_1_turn {
                        0 => {
                            84
                        },
                        _ => {
                            79
                        }
                    };
                    break;
                }
            }
            dummy_map.set(dummy_map_clone);
        })
    };

    let drop_disk_3 = {
        let game_map = game_map.clone();
        let dummy_map = dummy_map.clone();
        let current_turn = current_turn.clone();
        Callback::from(move |_| {
            let mut game_map_clone = (*game_map).clone();
            for i in 0..6 {
                if game_map_clone[5-i][2] == 0 {
                    game_map_clone[5-i][2] = match is_player_1_turn {
                        0 => {
                            1
                        },
                        _ => {
                            -1
                        }
                    };
                    current_turn.set((*current_turn) + 1);
                    break;
                }
            }
            game_map.set(game_map_clone);
            let mut dummy_map_clone = (*dummy_map).clone();
            for i in 0..6 {
                if dummy_map_clone[5-i][2] == 0 {
                    dummy_map_clone[5-i][2] = match is_player_1_turn {
                        0 => {
                            84
                        },
                        _ => {
                            79
                        }
                    };
                    break;
                }
            }
            dummy_map.set(dummy_map_clone);
        })
    };

    let drop_disk_4 = {
        let game_map = game_map.clone();
        let dummy_map = dummy_map.clone();
        let current_turn = current_turn.clone();
        Callback::from(move |_| {
            let mut game_map_clone = (*game_map).clone();
            for i in 0..6 {
                if game_map_clone[5-i][3] == 0 {
                    game_map_clone[5-i][3] = match is_player_1_turn {
                        0 => {
                            1
                        },
                        _ => {
                            -1
                        }
                    };
                    current_turn.set((*current_turn) + 1);
                    break;
                }
            }
            game_map.set(game_map_clone);
            let mut dummy_map_clone = (*dummy_map).clone();
            for i in 0..6 {
                if dummy_map_clone[5-i][3] == 0 {
                    dummy_map_clone[5-i][3] = match is_player_1_turn {
                        0 => {
                            84
                        },
                        _ => {
                            79
                        }
                    };
                    break;
                }
            }
            dummy_map.set(dummy_map_clone);
        })
    };

    let drop_disk_5 = {
        let game_map = game_map.clone();
        let dummy_map = dummy_map.clone();
        let current_turn = current_turn.clone();
        Callback::from(move |_| {
            let mut game_map_clone = (*game_map).clone();
            for i in 0..6 {
                if game_map_clone[5-i][4] == 0 {
                    game_map_clone[5-i][4] = match is_player_1_turn {
                        0 => {
                            1
                        },
                        _ => {
                            -1
                        }
                    };
                    current_turn.set((*current_turn) + 1);
                    break;
                }
            }
            game_map.set(game_map_clone);
            let mut dummy_map_clone = (*dummy_map).clone();
            for i in 0..6 {
                if dummy_map_clone[5-i][4] == 0 {
                    dummy_map_clone[5-i][4] = match is_player_1_turn {
                        0 => {
                            84
                        },
                        _ => {
                            79
                        }
                    };
                    break;
                }
            }
            dummy_map.set(dummy_map_clone);
        })
    };

    let drop_disk_6 = {
        let game_map = game_map.clone();
        let dummy_map = dummy_map.clone();
        let current_turn = current_turn.clone();
        Callback::from(move |_| {
            let mut game_map_clone = (*game_map).clone();
            for i in 0..6 {
                if game_map_clone[5-i][5] == 0 {
                    game_map_clone[5-i][5] = match is_player_1_turn {
                        0 => {
                            1
                        },
                        _ => {
                            -1
                        }
                    };
                    current_turn.set((*current_turn) + 1);
                    break;
                }
            }
            game_map.set(game_map_clone);
            let mut dummy_map_clone = (*dummy_map).clone();
            for i in 0..6 {
                if dummy_map_clone[5-i][5] == 0 {
                    dummy_map_clone[5-i][5] = match is_player_1_turn {
                        0 => {
                            84
                        },
                        _ => {
                            79
                        }
                    };
                    break;
                }
            }
            dummy_map.set(dummy_map_clone);
        })
    };

    let drop_disk_7 = {
        let game_map = game_map.clone();
        let dummy_map = dummy_map.clone();
        let current_turn = current_turn.clone();
        Callback::from(move |_| {
            let mut game_map_clone = (*game_map).clone();
            for i in 0..6 {
                if game_map_clone[5-i][6] == 0 {
                    game_map_clone[5-i][6] = match is_player_1_turn {
                        0 => {
                            1
                        },
                        _ => {
                            -1
                        }
                    };
                    current_turn.set((*current_turn) + 1);
                    break;
                }
            }
            game_map.set(game_map_clone);
            let mut dummy_map_clone = (*dummy_map).clone();
            for i in 0..6 {
                if dummy_map_clone[5-i][6] == 0 {
                    dummy_map_clone[5-i][6] = match is_player_1_turn {
                        0 => {
                            84
                        },
                        _ => {
                            79
                        }
                    };
                    break;
                }
            }
            dummy_map.set(dummy_map_clone);
        })
    };

    let updatep1name = {
        let pending_name_1 = pending_name_1.clone();
        Callback::from(move |name_event: InputEvent| {
            // DOM query from: https://stackoverflow.com/questions/71690906/how-to-query-and-update-the-dom-with-yew
            let event: Event = name_event.dyn_into().unwrap_throw();
            let input_elem: HtmlInputElement = event.target().unwrap_throw().dyn_into().unwrap_throw();
            let value = input_elem.value();
            pending_name_1.set(value);
        })
    };

    let updatep2name = {
        let pending_name_2 = pending_name_2.clone();
        Callback::from(move |name_event: InputEvent| {
            // DOM query from: https://stackoverflow.com/questions/71690906/how-to-query-and-update-the-dom-with-yew
            let event: Event = name_event.dyn_into().unwrap_throw();
            let input_elem: HtmlInputElement = event.target().unwrap_throw().dyn_into().unwrap_throw();
            let value = input_elem.value();
            pending_name_2.set(value);
        })
    };

    let toggle_colorblind = {
        let colorblind_enabled = colorblind_enabled.clone();
        let colorblind_enabled_display = colorblind_enabled_display.clone();
        Callback::from(move |_| {
            colorblind_enabled.set(!(*colorblind_enabled));
            colorblind_enabled_display.set(!(*colorblind_enabled));
        })
    };

    let start_game = {
        let is_game_on = is_game_on.clone();
        let is_canvas_drawn = is_canvas_drawn.clone();
        let disabled = disabled.clone();
        let display_state = display_state.clone();
        let canvas_context = canvas_context.clone();
        let player_name_1 = player_name_1.clone();
        let pending_name_1 = pending_name_1.clone();
        let player_name_2 = player_name_2.clone();
        let pending_name_2 = pending_name_2.clone();
        let player_name_1_display = player_name_1_display.clone();
        let player_name_2_display = player_name_2_display.clone();
        Callback::from(move |_| {
            is_game_on.set(true);
            disabled.set(true);
            display_state.set("block".to_string());

            // Remove black outline on final circle
            canvas_context.as_ref().unwrap().begin_path();
            canvas_context.as_ref().unwrap().set_fill_style(&"#00bfff".into());

            // Draw
            canvas_context.as_ref().unwrap().stroke();

            canvas_context.as_ref().unwrap();

            // Lock in names
            player_name_1.set((*pending_name_1).clone().to_string());
            player_name_2.set((*pending_name_2).clone().to_string());
            player_name_1_display.set((*pending_name_1).clone().to_string());
            player_name_2_display.set((*pending_name_2).clone().to_string());

            is_canvas_drawn.set(true);
        })
    };

    use_effect(move || {

        // Have a player win
        let win = |winner: i64| {
            let game_won = game_won.clone();
            game_won.set(true);
            let mut msg = String::new();

            let player_name_1 = player_name_1.clone();
            let player_name_2 = player_name_2.clone();
            let mut winner_num = 0;
            if winner > 0 {
                msg = format!("{} wins", (*player_name_1).clone());
                winner_num = 1;
            }
            else if winner < 0 {
                msg = format!("{} wins", (*player_name_2).clone());
                winner_num = 2;
            }
            else {
                msg = "It's a draw".to_string();
                winner_num = 0;
            }
        
            let print_msg = format!("{}", msg);
            
            let canvas_context = canvas_context.clone();
            canvas_context.as_ref().unwrap().save();
            canvas_context.as_ref().unwrap().set_font("14pt sans-serif");
            canvas_context.as_ref().unwrap().set_fill_style(&"#111".into());
            canvas_context.as_ref().unwrap().fill_text(&print_msg, 300.0, 20.0);
            canvas_context.as_ref().unwrap().restore();

            // send game to database
            let utc: DateTime<Utc> = Utc::now();
            let new_game = Game {
                id: 0,
                game_type_is_c4: true, 
                player_1_name: (*player_name_1).clone().to_string(),
                player_2_name: (*player_name_2).clone().to_string(),
                player_2_is_computer: false,
                player1_won: winner_num,
                date: utc
            };
            
            wasm_bindgen_futures::spawn_local(async move {
                reqwest::Client::new()
                    .post("http://127.0.0.1:7000/client")
                    .json(&new_game)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await.unwrap();
            });
        };

        let check = || {
            // Check if player won
            let mut right: i64 = 0;
            let mut down: i64 = 0;
            let mut down_right: i64 = 0;
            let mut up_right: i64 = 0;
            let mut right1 = vec![0; 4];
            let mut down1 = vec![0; 4];
            let mut down_right1 = vec![0; 4];
            let mut up_right1 = vec![0; 4];
            
            let dummy_map = dummy_map.clone();
            for i in 0..6 {
                for j in 0..7 {
                    for k in 0..4 {
                        right1[k] = 0;
                        down1[k] = 0;
                        down_right1[k] = 0;
                        up_right1[k] = 0;
                    }
                    for k in 0..4 {
                        if j + k < 7 {
                            right1[k] = (*dummy_map)[i][j + k];
                        }
                        if i + k < 6 {
                            down1[k] = (*dummy_map)[i + k][j];
                        }
                        if i + k < 6 && j + k < 7 {
                            down_right1[k] = (*dummy_map)[i + k][j + k];
                        }
                        if i >= k && j + k < 7 {
                            up_right1[k] = (*dummy_map)[i - k][j + k];
                        }
                    }
        
                    if right1[0] == 84 && right1[1] == 79 && right1[2] == 79 && right1[3] == 84 {
                        win(1);
                    }
                    else if right1[0] == 79 && right1[1] == 84 && right1[2] == 84 && right1[3] == 79 {
                        win(-1);
                    }
                    else if down1[0] == 84 && down1[1] == 79 && down1[2] == 79 && down1[3] == 84 {
                        win(1);
                    }
                    else if down1[0] == 79 && down1[1] == 84 && down1[2] == 84 && down1[3] == 79 {
                        win(-1);
                    }
                    else if down_right1[0] == 84 && down_right1[1] == 79 && down_right1[2] == 79 && down_right1[3] == 84 {
                        win(1);
                    }
                    else if down_right1[0] == 79 && down_right1[1] == 84 && down_right1[2] == 84 && down_right1[3] == 79 {
                        win(-1);
                    }
                    else if up_right1[0] == 84 && up_right1[1] == 79 && up_right1[2] == 79 && up_right1[3] == 84 {
                        win(1);
                    }
                    else if up_right1[0] == 79 && up_right1[1] == 84 && up_right1[2] == 84 && up_right1[3] == 79 {
                        win(-1);
                    }
                }
            }
            
            // check if the game is a tie
            let current_turn = current_turn.clone();
            let game_won = game_won.clone();
            if (*current_turn == 42) && (!*game_won) {
                win(0);
            }
        };
        
        if is_mounted() && !*canvas_context_exists {
            let canvas_context_exists = canvas_context_exists.clone();
            let canvas = canvas.clone();
            let canvas_context = canvas_context.clone();

            canvas_context_exists.set(true);
            canvas_context.set(Some(get_canvas_context()));
            canvas.set(Some(get_canvas_element()));
        }

        // Draw the gameboard on every re-render
        if *is_canvas_drawn && !*game_won {
            // Draw gameboard on every re-render
            for i in 0..6 { // y coord
                for j in 0..7 { // x coord
                    canvas_context.as_ref().unwrap().begin_path();
                    canvas_context.as_ref().unwrap().set_fill_style(&"#00bfff".into());
                    canvas_context.as_ref().unwrap().fill_rect(
                        (75 * j + 150) as f64,
                        (75 * i) as f64,
                        -100.0,
                        100.0,
                    );
                    canvas_context.as_ref().unwrap().fill();
                    let circle_color = match (*game_map)[i][j] {
                        0 => {
                            "#ffffff"
                        },
                        1 => {
                            match (*colorblind_enabled).clone() {
                                true => {
                                    "#5D3A9B"
                                },
                                false => {
                                    "#ff4136"
                                }
                            }
                        }
                        _ => {
                            match (*colorblind_enabled).clone() {
                                true => {
                                    "#E66100"
                                },
                                false => {
                                    "#ffff00"
                                }
                            }
                        }
                    };
                    canvas_context.as_ref().unwrap().set_fill_style(&circle_color.into());
                    canvas_context.as_ref().unwrap().arc(
                        (75 * j + 100) as f64,
                        (75 * i + 50) as f64,
                        25.0,
                        0.0,
                        2.0 * 3.14159265359,
                    );
                    canvas_context.as_ref().unwrap().fill();
                    if circle_color == "#ff4136" || circle_color == "#5D3A9B" {
                        canvas_context.as_ref().unwrap().save();
                        canvas_context.as_ref().unwrap().set_font("bold 25px serif");
                        canvas_context.as_ref().unwrap().set_fill_style(&"#111".into());
                        canvas_context.as_ref().unwrap().fill_text("T", (75 * j + 100) as f64 - 8.5, (75 * i + 50) as f64 + 8.0);
                        canvas_context.as_ref().unwrap().restore();
                    }
                    else if circle_color == "#ffff00" || circle_color == "#E66100" {
                        canvas_context.as_ref().unwrap().save();
                        canvas_context.as_ref().unwrap().set_font("bold 25px serif");
                        canvas_context.as_ref().unwrap().set_fill_style(&"#111".into());
                        canvas_context.as_ref().unwrap().fill_text("O", (75 * j + 100) as f64 - 8.5, (75 * i + 50) as f64 + 8.0);
                        canvas_context.as_ref().unwrap().restore();
                    }
                }
            }

            if !*game_won {
                // Check if player has won
                check();
            }
        }

        move || ()
    });
    
    html! {
        <>
            <div class="name-entry-container">
                <input
                    class="name-textbox"
                    type="text"
                    placeholder="Player 1 Name"
                    oninput={updatep1name}
                />
                <input
                    class="name-textbox"
                    type="text"
                    placeholder="Player 2 Name"
                    oninput={updatep2name}
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
                <button
                    class="button toggle-colorblind"
                    type="button"
                    onclick={toggle_colorblind}
                >
                {"Toggle Colorblind Mode"}
                </button>
            </div>
            if *is_game_on {
                <div style={format!("display: {}", *display_state)}>
                    <br/>
                    <h4>{format!("New Game: {} Vs {}", (*player_name_1_display).clone(), (*player_name_2_display).clone())}</h4>
                    <small>{format!("(Disc Colors: {} - ", (*player_name_1_display).clone())} <b>{
                        match (*colorblind_enabled_display).clone() {
                            true => {
                                "Purple"
                            },
                            false => {
                                "Red"
                            }
                    }}</b> {format!("   and    {} - ", (*player_name_2_display).clone())} <b>{
                        match (*colorblind_enabled_display).clone() {
                            true => {
                                "Orange"
                            },
                            false => {
                                "Yellow"
                            }
                    }}</b>{")"}</small>
                    <br/>
                </div>
            }
            <br/>
            <canvas id="canvas" height="480" width="640"></canvas>
            if *is_game_on {
                <div class="button-container">
                    <button class="button canvas-button" type="button" onclick={drop_disk_1}> {"Drop"} </button>
                    <button class="button canvas-button" type="button" onclick={drop_disk_2}> {"Drop"} </button>
                    <button class="button canvas-button" type="button" onclick={drop_disk_3}> {"Drop"} </button>
                    <button class="button canvas-button" type="button" onclick={drop_disk_4}> {"Drop"} </button>
                    <button class="button canvas-button" type="button" onclick={drop_disk_5}> {"Drop"} </button>
                    <button class="button canvas-button" type="button" onclick={drop_disk_6}> {"Drop"} </button>
                    <button class="button canvas-button" type="button" onclick={drop_disk_7}> {"Drop"} </button>
                </div>
            }
        </>
    }
}
