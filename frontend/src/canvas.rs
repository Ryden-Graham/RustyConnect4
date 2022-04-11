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
// use yew::format::Json;
// use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{prelude::*, virtual_dom::VNode, Properties};
use log;
use yew_hooks::use_is_mounted;
use crate::connect4Computer::Difficulty::{self, *};
use crate::ScoreBoard::Game;

// macro_rules! enclose {
//     ( ($( $x:ident ),*) $y:expr ) => {
//         {
//             $(let $x = $x.clone();)*
//             $y
//         }
//     };
// }

#[derive(Clone, PartialEq, Properties)]
pub struct CanvasProps {
    // pub player1: Option<String>,
    // pub player2: Option<String>,
    // pub difficulty: Difficulty,
    // pub canvas_id: Option<String>,
    // pub game_done_call_back_click: Callback<i64>,
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

#[function_component(CanvasModel)]
pub fn canvasModel(props: &CanvasProps) -> Html {
    // Boolean check state variables
    let is_mounted = use_is_mounted();
    let canvas_context_exists = use_state(|| false);
    let is_canvas_drawn = use_state(|| false);
    let is_game_on = use_state(|| false);
    let is_listener_active = use_state(|| false);
    let disabled = use_state(|| false);
    
    // Complex state variables
    let canvas_context:UseStateHandle<Option<web_sys::CanvasRenderingContext2d>> = use_state(|| None);
    let canvas:UseStateHandle<Option<web_sys::HtmlCanvasElement>> = use_state(|| None);
    let player_name = use_state(|| "".to_string());
    let display_state = use_state(|| "".to_string());
    let game_map = use_state(|| vec![vec![0; 7]; 6]);

    let is_player_1_turn:usize = (*game_map).clone().iter().map(|column| column.iter().filter(|circle_number| **circle_number != 0).count()).sum::<usize>() % 2;

    fn on_region (coord: f64, x: f64, radius: f64) -> bool {
        return ((coord - x) * (coord - x) <= radius * radius);
    };

    // Add piece
    let game_map_state = game_map.clone();
    
    let drop_disk_1 = {
        let game_map = game_map.clone();
        Callback::from(move |_| {
            let mut game_map_clone = (*game_map).clone();
            for i in 0..6 {
                if game_map_clone[5-i][0] == 0 {
                    game_map_clone[5-i][0] = match is_player_1_turn {
                        0 => {
                            1
                        },
                        _ => {
                            2
                        }
                    };
                    break;
                }
            }
            game_map.set(game_map_clone);
        })
    };

    let drop_disk_2 = {
        let game_map = game_map.clone();
        Callback::from(move |_| {
            let mut game_map_clone = (*game_map).clone();
            for i in 0..6 {
                if game_map_clone[5-i][1] == 0 {
                    game_map_clone[5-i][1] = match is_player_1_turn {
                        0 => {
                            1
                        },
                        _ => {
                            2
                        }
                    };
                    break;
                }
            }
            game_map.set(game_map_clone);
        })
    };

    let drop_disk_3 = {
        let game_map = game_map.clone();
        Callback::from(move |_| {
            let mut game_map_clone = (*game_map).clone();
            for i in 0..6 {
                if game_map_clone[5-i][2] == 0 {
                    game_map_clone[5-i][2] = match is_player_1_turn {
                        0 => {
                            1
                        },
                        _ => {
                            2
                        }
                    };
                    break;
                }
            }
            game_map.set(game_map_clone);
        })
    };

    let drop_disk_4 = {
        let game_map = game_map.clone();
        Callback::from(move |_| {
            let mut game_map_clone = (*game_map).clone();
            for i in 0..6 {
                if game_map_clone[5-i][3] == 0 {
                    game_map_clone[5-i][3] = match is_player_1_turn {
                        0 => {
                            1
                        },
                        _ => {
                            2
                        }
                    };
                    break;
                }
            }
            game_map.set(game_map_clone);
        })
    };

    let drop_disk_5 = {
        let game_map = game_map.clone();
        Callback::from(move |_| {
            let mut game_map_clone = (*game_map).clone();
            for i in 0..6 {
                if game_map_clone[5-i][4] == 0 {
                    game_map_clone[5-i][4] = match is_player_1_turn {
                        0 => {
                            1
                        },
                        _ => {
                            2
                        }
                    };
                    break;
                }
            }
            game_map.set(game_map_clone);
        })
    };

    let drop_disk_6 = {
        let game_map = game_map.clone();
        Callback::from(move |_| {
            let mut game_map_clone = (*game_map).clone();
            for i in 0..6 {
                if game_map_clone[5-i][5] == 0 {
                    game_map_clone[5-i][5] = match is_player_1_turn {
                        0 => {
                            1
                        },
                        _ => {
                            2
                        }
                    };
                    break;
                }
            }
            game_map.set(game_map_clone);
        })
    };

    let drop_disk_7 = {
        let game_map = game_map.clone();
        Callback::from(move |_| {
            let mut game_map_clone = (*game_map).clone();
            for i in 0..6 {
                if game_map_clone[5-i][6] == 0 {
                    game_map_clone[5-i][6] = match is_player_1_turn {
                        0 => {
                            1
                        },
                        _ => {
                            2
                        }
                    };
                    break;
                }
            }
            game_map.set(game_map_clone);
        })
    };

    // let add_piece = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
    //     let mut game_map_clone = (*game_map_state).clone();
    //     for i in 0..7 {
    //         if (event.offset_x() as f64) < (640.0/7.0*((i+1) as f64)) as f64 {
    //             for j in 0..6 {
    //                 if game_map_clone[5-j][i] == 0 {
    //                     game_map_clone[5-j][i] = match is_player_1_turn {
    //                         0 => {
    //                             1
    //                         },
    //                         _ => {
    //                             2
    //                         }
    //                     };
    //                     break;
    //                 }
    //             }
    //             break;
    //         }
    //     }

    //     game_map_state.set(game_map_clone);
    //     // canvas_context_add.as_ref().unwrap().move_to(event.offset_x() as f64, event.offset_y() as f64);
    // }) as Box<dyn FnMut(_)>);


    let start_game = {
        let is_game_on = is_game_on.clone();
        let is_canvas_drawn = is_canvas_drawn.clone();
        let disabled = disabled.clone();
        let display_state = display_state.clone();
        let canvas_context = canvas_context.clone();

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

            is_canvas_drawn.set(true);
        })
    };

    use_effect(move || {
        if is_mounted() && !*canvas_context_exists {
            let canvas_context_exists = canvas_context_exists.clone();
            let canvas = canvas.clone();
            let canvas_context = canvas_context.clone();

            canvas_context_exists.set(true);
            canvas_context.set(Some(get_canvas_context()));
            canvas.set(Some(get_canvas_element()));
        }

        // Draw the gameboard on every re-render
        if *is_canvas_drawn {
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
                            "#ff4136"
                        }
                        _ => {
                            "#ffff00"
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
                }
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
            if *is_game_on {
                <div style={format!("display: {}", *display_state)}>
                    <br/>
                    <h4>{format!("New Game: {} Vs Computer", *player_name)}</h4>
                    <small>{format!("(Disc Colors: {} - ", *player_name)} <b>{"Red"}</b> {"   and    Computer - "} <b>{"Yellow)"}</b></small>
                    <br/>
                </div>
            }
            <br/>
            <canvas id="canvas" height="480" width="640"></canvas>
            if *is_game_on {
                <button class="button canvas-button" type="button" onclick={drop_disk_1}> {"Drop"} </button>
                <button class="button canvas-button" type="button" onclick={drop_disk_2}> {"Drop"} </button>
                <button class="button canvas-button" type="button" onclick={drop_disk_3}> {"Drop"} </button>
                <button class="button canvas-button" type="button" onclick={drop_disk_4}> {"Drop"} </button>
                <button class="button canvas-button" type="button" onclick={drop_disk_5}> {"Drop"} </button>
                <button class="button canvas-button" type="button" onclick={drop_disk_6}> {"Drop"} </button>
                <button class="button canvas-button" type="button" onclick={drop_disk_7}> {"Drop"} </button>
            }
        </>
    }
}
