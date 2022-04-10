use std::error::Error;
use std::ptr::null;
use stdweb::traits::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use stdweb::unstable::TryInto;
use stdweb::js;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::Date;
use stdweb::web::FillRule;
use stdweb::web::{document, window, CanvasRenderingContext2d};
use stdweb::web::event::ClickEvent;
use yew::{prelude::*, virtual_dom::VNode, Properties};
use log::info;
use yew_hooks::use_is_mounted;
use crate::connect4Computer::Difficulty::{self, *};
use crate::ScoreBoard::Game;
use web_sys::HtmlInputElement;

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
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
pub fn canvasModel() -> Html {
    // Boolean check state variables
    let is_mounted = use_is_mounted();
    let canvas_context_exists = use_state(|| false);
    let is_canvas_drawn = use_state(|| false);
    let is_game_on = use_state(|| false);
    let is_listener_active = use_state(|| false);
    let disabled = use_state(|| false);
    let game_won = use_state(|| false);
    let paused = use_state(|| false);
    let reject_click = use_state(|| false);
    
    // Complex state variables
    let canvas_context:UseStateHandle<Option<web_sys::CanvasRenderingContext2d>> = use_state(|| None);
    let canvas:UseStateHandle<Option<web_sys::HtmlCanvasElement>> = use_state(|| None);
    let player_name_1 = use_state(|| "".to_string());
    let player_name_2 = use_state(|| "Computer".to_string());
    let display_state = use_state(|| "".to_string());
    let map = use_state(|| vec![vec![0; 7]; 6]);
    let current_turn = use_state(|| 0);
    let difficulty = use_state(|| Difficulty::Easy);

    let value;
    let choose;
    let ai;
    let animate;

    let clear = || {
        let canvas_context = canvas_context.clone();
        let canvas = canvas.clone();
        canvas_context.as_ref().unwrap().clear_rect(0.0, 0.0, (*canvas).as_ref().unwrap().width() as f64, (*canvas).as_ref().unwrap().height() as f64);
    };

    let draw_mask = || {
        let canvas_context = canvas_context.clone();
        canvas_context.as_ref().unwrap().save();
        canvas_context.as_ref().unwrap().set_fill_style(&"#00bfff".into());
        canvas_context.as_ref().unwrap().begin_path();
        for y in 0..6 {
            for x in 0..7 {
                canvas_context.as_ref().unwrap().arc((75 * x + 100) as f64, (75 * y + 50) as f64, 25.0, 0.0, 2.0 * std::f64::consts::PI);
                canvas_context.as_ref().unwrap().rect((75 * x + 150) as f64, (75 * y) as f64, -100.0, 100.0);
            }
        }
        canvas_context.as_ref().unwrap().fill();
        canvas_context.as_ref().unwrap().restore();
    };

    let draw_circle = |x: u32, y: u32, r: u32, fill: &str, stroke: &str| {
        let canvas_context = canvas_context.clone();
        canvas_context.as_ref().unwrap().save();
        canvas_context.as_ref().unwrap().set_fill_style(&fill.into());
        canvas_context.as_ref().unwrap().set_stroke_style(&stroke.into());
        canvas_context.as_ref().unwrap().begin_path();
        canvas_context.as_ref().unwrap().arc(x as f64, y as f64, 25.0, 0.0, 2.0 * std::f64::consts::PI);
        canvas_context.as_ref().unwrap().fill();
        canvas_context.as_ref().unwrap().restore();
    };
    
    let draw = || {
        let map = map.clone();
        for y in 0..6 {
            for x in 0..7 {
                let mut fg_color = "transparent";
                if (*map)[y][x] >= 1 {
                    fg_color = "#ff4136";
                }
                else if (*map)[y][x] <= -1 {
                    fg_color = "#ffff00";
                }
                draw_circle((75 * x + 100) as u32, (75 * y + 50) as u32, 25, &fg_color, "black");
            }
        }
    };
    
    let reset = || {
        let map = map.clone();
        map.set(vec![vec![0; 7]; 6]);
        let paused = paused.clone();
        paused.set(false);
        let game_won = game_won.clone();
        game_won.set(false);
        let reject_click = reject_click.clone();
        reject_click.set(false);
        let current_turn = current_turn.clone();
        current_turn.set(0);
        clear();
        draw_mask();
    };
    
    let player_move = || -> i64 {
        let current_turn = current_turn.clone();
        if (*current_turn) % 2 == 0 {
            return 1;
        }
        return -1;
    };
    
    let win = |winner: i64| {
        let paused = paused.clone();
        paused.set(true);
        let game_won = game_won.clone();
        game_won.set(true);
        let reject_click = reject_click.clone();
        reject_click.set(false);
        let mut msg = String::new();

        let player_name_1 = player_name_1.clone();
        let player_name_2 = player_name_2.clone();
        if winner > 0 {
            msg = format!("{} wins", (*player_name_1));
        }
        else if winner < 0 {
            msg = format!("{} wins", (*player_name_2));
        }
        else {
            msg = "It's a draw".to_string();
        }
    
        let print_msg = format!("{} - Click on game board to reset", msg);
        
        let canvas_context = canvas_context.clone();
        canvas_context.as_ref().unwrap().save();
        canvas_context.as_ref().unwrap().set_font("14pt sans-serif");
        canvas_context.as_ref().unwrap().set_fill_style(&"#111".into());
        canvas_context.as_ref().unwrap().fill_text(&print_msg, 130.0, 20.0);
        canvas_context.as_ref().unwrap().restore();
    
        // let game = Game {
        //     gameNumber: String::new(),
        //     gameType: String::from("Connect-4"),
        //     Player1Name: self.props.player1.as_ref().unwrap().clone(),
        //     Player2Name: self.props.player2.as_ref().unwrap().clone(),
        //     WinnerName: if winner > 0 {
        //         self.props.player1.as_ref().unwrap().clone()
        //     }
        //     else if winner < 0 {
        //         self.props.player2.as_ref().unwrap().clone()
        //     }
        //     else {
        //         String::from("Draw")
        //     },
        //     GameDate: Date::now() as u64,
        // };
    
        // // construct callback
        // let callback = self
        //     .link
        //     .callback(move |response: Response<Result<String, Error>>| {
        //         log::info!("successfully saved!");
        //         Message::Ignore
        //     });
    
        // // construct request
        // let request = Request::post("/games")
        //     .header("Content-Type", "application/json")
        //     .body(Json(&game))
        //     .unwrap();
    
        // // send the request
        // self.fetch_task = self.fetch_service.fetch(request, callback).ok();
    };
    
    let fill_map = |state: &Vec<Vec<i64>>, column: usize, value: i64| -> Vec<Vec<i64>> {
        let mut new_board = state.clone();
        if new_board[0][column] != 0 || column < 0|| column > 6 {
            new_board[0][column] = -999; // error detection
        }
    
        let mut done = false;
        let mut row = 0;
    
        for i in (0..6).rev() {
            if new_board[i][column] == 0 {
                done = true;
                row = i;
                break;
            }
        }
    
        new_board[row][column] = value;
        return new_board;
    };

    let check = || {
        let mut right: i64 = 0;
        let mut down: i64 = 0;
        let mut down_right: i64 = 0;
        let mut up_right: i64 = 0;
        
        let map = map.clone();
        for i in 0..6 {
            for j in 0..7 {
                right = 0;
                down = 0;
                down_right = 0;
                up_right = 0;
                for k in 0..4 {
                    if j + k < 7 {
                        right += (*map)[i][j + k];
                    }
                    if i + k < 6 {
                        down += (*map)[i + k][j];
                    }
                    if i + k < 6 && j + k < 7 {
                        down_right += (*map)[i + k][j + k];
                    }
                    if i >= k && j + k < 7 {
                        up_right += (*map)[i - k][j + k];
                    }
                }

                if right.abs() == 4 {
                    win(right);
                } 
                else if down.abs() == 4 {
                    win(down);
                } 
                else if down_right.abs() == 4 {
                    win(down_right);
                } 
                else if up_right.abs() == 4 {
                    win(up_right);
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

    let action = |column: usize, callback: bool| -> i64 {
        let paused = paused.clone();
        let game_won = game_won.clone();
        if *paused || *game_won {
            return 0;
        }
        
        let map = map.clone();
        if (*map)[0][column] != 0 || column < 0 || column > 6 {
            return -1;
        }
    
        let mut done = false;
        let mut row = 0;
        for i in (0..6).rev() {
            if (*map)[i][column] == 0 {
                done = true;
                row = i;
                break;
            }
        }

        animate(column, player_move(), row, 0, callback);
    
        paused.set(true);
        return 1;
    };
    
    let on_region = |coord: f64, x: f64, radius: f64| -> bool {
        return ((coord - x) * (coord - x) <= radius * radius);
    };
    
    let check_state = |state: &Vec<Vec<i64>>| -> (i64, i64) {
        let mut win_val = 0;
        let mut chain_val = 0;
        let mut right = 0;
        let mut down = 0;
        let mut down_right = 0;
        let mut up_right = 0;
        for i in 0..6 {
            for j in 0..7 {
                right = 0;
                down = 0;
                down_right = 0;
                up_right = 0;
                for k in 0..4 {
                    if j + k < 7 {
                        right += state[i][j + k];
                    }
                    if i + k < 6 {
                        down += state[i + k][j];
                    }
                    if i + k < 6 && j + k < 7 {
                        down_right += state[i + k][j + k];
                    }
                    if i >= k && j + k < 7 {
                        up_right += state[i - k][j + k];
                    }
                }
                chain_val += right * right * right;
                chain_val += down * down * down;
                chain_val += down_right * down_right * down_right;
                chain_val += up_right * up_right * up_right;
    
                if right.abs() == 4 {
                    win_val = right;
                }
                else if down.abs() == 4 {
                    win_val = down;
                }
                else if down_right.abs() == 4 {
                    win_val = down_right;
                }
                else if up_right.abs() == 4 {
                    win_val = up_right;
                }
            }
        }
        return (win_val, chain_val);
    };
    
    let max_state = |ai_move_value: i64, state: &Vec<Vec<i64>>, depth: i64, mut alpha: i64, mut beta: i64| -> (i64, i64) {
        let mut v = -100000000007;
        let mut next_move: i64 = -1;
        let mut move_queue = Vec::new();
        
        for j in 0..7 {
            let temp_state = fill_map(state, j, ai_move_value);
            if temp_state[0][j] != -999 {
                let temp_val = value(ai_move_value, &temp_state, depth, alpha, beta);
                if temp_val.0 > v {
                    v = temp_val.0;
                    next_move = j as i64;
                    move_queue = Vec::new();
                    move_queue.push(j);
                }
                else if temp_val.0 == v {
                    move_queue.push(j);
                }
    
                // alpha-beta pruning
                if v > beta {
                    next_move = choose(&move_queue);
                    return (v, next_move);
                }
                alpha = std::cmp::max(alpha, v);
            }
        }
        next_move = choose(&move_queue);
    
        return (v, next_move);
    };
    
    let min_state = |ai_move_value: i64, state: &Vec<Vec<i64>>, depth: i64, mut alpha: i64, mut beta: i64| -> (i64, i64) {
        let mut v = 100000000007;
        let mut next_move: i64 = -1;
        let mut move_queue = Vec::new();
    
        for j in 0..7 {
            let temp_state = fill_map(state, j, ai_move_value * -1);
            if temp_state[0][j] != -999 {
                let temp_val = value(ai_move_value, &temp_state, depth, alpha, beta);
                if temp_val.0 < v {
                    v = temp_val.0;
                    next_move = j as i64;
                    move_queue = Vec::new();
                    move_queue.push(j);
                }
                else if temp_val.0 == v {
                    move_queue.push(j);
                }
    
                // alpha-beta pruning
                if v < alpha {
                    next_move = choose(&move_queue);
                    return (v, next_move);
                }
                beta = std::cmp::min(beta, v);
            }
        }
        next_move = choose(&move_queue);
    
        return (v, next_move);
    };

    let value = |ai_move_value: i64, state: &Vec<Vec<i64>>, depth: i64, mut alpha: i64, mut beta: i64| -> (i64, i64) {
        let val = check_state(state);
        // make depth lower if ai is harder
        let difficulty = difficulty.clone();
        let max_depth = match *difficulty {
            Easy => 1,
            Medium => 3,
            Hard => 5,
        };
        if depth >= max_depth {
            // calculate value
            let mut ret_val = 0;
    
            // if win, value = +inf
            let win_val = val.0;
            let chain_val = val.1 * ai_move_value;
            ret_val = chain_val;
    
            // If it lead to winning, then do it
            if win_val == 4 * ai_move_value { // AI win, AI wants to win of course
                ret_val = 999999;
            }
            else if win_val == 4 * ai_move_value * -1 { // AI lose, AI hates losing
                ret_val = 999999 * -1;
            }
            ret_val -= depth * depth;
    
            return (ret_val, -1);
        }
    
        let win = val.0;
        // if already won, then return the value right away
        if win == 4 * ai_move_value { // AI win, AI wants to win of course
            return (999999 - depth * depth, -1);
        }
        if win == 4 * ai_move_value * -1 { // AI lose, AI hates losing
            return (999999 * -1 - depth * depth, -1);
        }
    
        if depth % 2 == 0 {
            return min_state(ai_move_value, state, depth + 1, alpha, beta);
        }
        return max_state(ai_move_value, state, depth + 1, alpha, beta);
    };

    let get_random_val = |val: usize| -> usize {
        let rand_num = js! { return Math.random(); };
        let rand_num_f64: f64 = stdweb::unstable::TryInto::try_into(rand_num).unwrap();
        return (rand_num_f64 * val as f64).floor() as usize;
    };
    
    let choose = |choice: &Vec<usize>| -> i64 {
        let index = get_random_val(choice.len());
        return choice[index] as i64;
    };

    struct ai<'s> { ai: &'s dyn Fn(&ai, i64) }

    let ai = ai {
        ai: &|fact, ai_move_value| {
            let map = map.clone();
            let new_map = (*map).clone();
            let choice_val = max_state(ai_move_value, &new_map, 0, -100000000007, 100000000007);
        
            let val = choice_val.0;
            let choice = choice_val.1;
            let paused = paused.clone();
            paused.set(false);
            let mut done = action(choice as usize, true);
        
            // if fail, then random
            while done < 0 {
                log::info!("Falling back to random agent");
                let random_choice = get_random_val(7);
                let reject_click = reject_click.clone();
                done = action(random_choice, true);
            }
        }
    };

    // let ai = |ai_move_value: i64| {
    //     let map = map.clone();
    //     let new_map = (*map).clone();
    //     let choice_val = max_state(ai_move_value, &new_map, 0, -100000000007, 100000000007);
    
    //     let val = choice_val.0;
    //     let choice = choice_val.1;
    //     let paused = paused.clone();
    //     paused.set(false);
    //     let mut done = action(choice as usize, true);
    
    //     // if fail, then random
    //     while done < 0 {
    //         log::info!("Falling back to random agent");
    //         let random_choice = get_random_val(7);
    //         let reject_click = reject_click.clone();
    //         done = action(random_choice, true);
    //     }
    // };

    let animate = |column: usize, turn: i64, to_row: usize, cur_pos: usize, callback: bool| {
        let mut fg_color = "transparent";
        if turn >= 1 {
            fg_color = "#ff4136";
        }
        else if turn <= -1 {
            fg_color = "#ffff00";
        }
    
        if to_row * 75 >= cur_pos {
            clear();
            draw();
            draw_circle((75 * column + 100) as u32, (cur_pos + 50) as u32, 25, &fg_color, "black");
            draw_mask();
    
            // let cloned = self.animate_call_back_click.clone();
            // window().request_animation_frame(enclose!((cloned) move |_| {
            //     cloned.emit((column, current_turn, to_row, cur_pos + 25, callback));
            // }));
        }
        else {
            (*map)[to_row][column] = player_move();
            current_turn.set(*current_turn + 1);
            draw();
            check();
            let player_name_2 = player_name_2.clone();
            if callback == false && (*player_name_2) == "Computer" {
                (ai.ai)(&ai, -1);
            } else {
                reject_click.set(false);
            }
        }
    };

    // Add piece
    let canvas_context_add = canvas_context.clone();
    let canvas = canvas.clone();
    let add_piece = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        canvas_context_add.as_ref().unwrap().begin_path();
        canvas_context_add.as_ref().unwrap().set_fill_style(&"#ff4136".into());
        let rect = canvas.as_ref().unwrap().get_bounding_client_rect();
        let x = event.offset_x() as f64 - rect.left();
        for j in 0..7 {
            if on_region(x, (75 * j + 100) as f64, 25 as f64) {
                paused.set(false);

                let valid = action(j, false);
                if valid == 1 {
                    reject_click.set(true);
                };

                break;
            }
        }
        canvas_context_add.as_ref().unwrap().fill(); 
        // canvas_context_add.as_ref().unwrap().move_to(event.offset_x() as f64, event.offset_y() as f64);
    }) as Box<dyn FnMut(_)>);

    let get_name = {
        let player_name_1 = player_name_1.clone();
        Callback::from(move |name_event: InputEvent| {
            // DOM query from: https://stackoverflow.com/questions/71690906/how-to-query-and-update-the-dom-with-yew
            let event: Event = name_event.dyn_into().unwrap_throw();
            let input_elem: HtmlInputElement = event.target().unwrap_throw().dyn_into().unwrap_throw();
            let value = input_elem.value();
            player_name_1.set(value);
        })
    };

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
            
            // Draw the gameboard
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
                    canvas_context.as_ref().unwrap().set_fill_style(&"#ffffff".into());
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

        if *is_canvas_drawn && !*is_listener_active {
            let is_listener_active = is_listener_active.clone();
            is_listener_active.set(true);
            let canvas = canvas.clone();

            canvas.as_ref().unwrap().add_event_listener_with_callback("click", add_piece.as_ref().unchecked_ref());
            add_piece.forget();
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
                    oninput={get_name}
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
                    <h4>{format!("New Game: {} Vs Computer", *player_name_1)}</h4>
                    <small>{format!("(Disc Colors: {} - ", *player_name_1)} <b>{"Red"}</b> {"   and    Computer - "} <b>{"Yellow)"}</b></small>
                    <br/>
                    
                        // canvas_id = "connect_computer" 
                        // player1 = {*player_name.clone()}
                        // player2 = "Computer" 
                        // difficulty = self.difficulty,
                        // game_done_cbk={end_game}/>
                </div>
            }
            <br/>
            <canvas id="canvas" height="480" width="640"></canvas>
        </>
    }
}
    

