// use std::error::Error
// use stdweb::traits::*;
// use stdweb::web::html_element::CanvasElement;
// use stdweb::web::Date;
// use stdweb::web::FillRule;
// use stdweb::web::{document, window, CanvasRenderingContext2d};
// use yew::format::Json;
// use yew::services::fetch::{FetchService, FetchTask, Request, Response};
// use yew::{prelude::*, virtual_dom::VNode, Properties};

// use crate::Connect4Computer::Difficulty::{self, *};
// use crate::ScoreBoard::Game;

// macro_rules! enclose {
//     ( ($( $x:ident ),*) $y:expr ) => {
//         {
//             $(let $x = $x.clone();)*
//             $y
//         }
//     };
// }

// pub struct CanvasModel {
//     props: Props,
//     canvas_id: String,
//     canvas: Option<CanvasElement>,
//     canvas_context: Option<CanvasRenderingContext2d>,
//     call_back_click: Callback<ClickEvent>,
//     animate_call_back_click: Callback<(usize, i64, usize, usize, bool)>,
//     map: Vec<Vec<i64>>,
//     current_turn: i64,
//     won: bool,
//     paused: bool,
//     reject_click: bool,
//     fetch_service: FetchService,
//     fetch_task: Option<FetchTask>,
//     link: ComponentLink<CanvasModel>,
// }

// #[derive(Clone, PartialEq, Properties)]
// pub struct Props {
//     pub player1: Option<String>,
//     pub player2: Option<String>,
//     pub difficulty: Difficulty,
//     pub canvas_id: Option<String>,
//     pub game_done_call_back_click: Callback<i64>,
// }

// pub enum Message {
//     Click(ClickEvent),
//     AnimateCallback((usize, i64, usize, usize, bool)),
//     Ignore,
// }

// impl CanvasModel {
//     pub fn reset(&mut self) {
//         self.map = vec![vec![0; 7]; 6];
//         self.current_turn = 0;
//         self.paused = false;
//         self.won = false;
//         self.reject_click = false;
//         self.clear();
//         self.draw_mask();
//     }

//     #[inline]
//     pub fn check_state(&self, state: &Vec<Vec<i64>>) -> (i64, i64) {
//         let mut win_val = 0;
//         let mut chain_val = 0;
//         let (mut right, mut down, mut down_right, mut up_right) = (0, 0, 0, 0);
//         for i in 0..6 {
//             for j in 0..7 {
//                 right = 0;
//                 down = 0;
//                 down_right = 0;
//                 up_right = 0;
//                 for k in 0..4 {
//                     if j + k < 7 {
//                         right += state[i][j + k];
//                     }

//                     if i + k < 6 {
//                         down += state[i + k][j];
//                     }

//                     if i + k < 6 && j + k < 7 {
//                         down_right += state[i + k][j + k];
//                     }

//                     if i >= k && j + k < 7 {
//                         up_right += state[i - k][j + k];
//                     }
//                 }
//                 chain_val += right * right * right;
//                 chain_val += down * down * down;
//                 chain_val += down_right * down_right * down_right;
//                 chain_val += up_right * up_right * up_right;

//                 if right.abs() == 4 {
//                     win_val = right;
//                 } else if down.abs() == 4 {
//                     win_val = down;
//                 } else if down_right.abs() == 4 {
//                     win_val = down_right;
//                 } else if up_right.abs() == 4 {
//                     win_val = up_right;
//                 }
//             }
//         }

//         return (win_val, chain_val);
//     }

//     pub fn value(
//         &self,
//         ai_move_value: i64,
//         state: &Vec<Vec<i64>>,
//         depth: i64,
//         mut alpha: i64,
//         mut beta: i64,
//     ) -> (i64, i64) {
//         let val = self.check_state(state);
//         let max_depth = match self.props.difficulty {
//             Easy => 1,
//             Medium => 3,
//             Hard => 5,
//         };
//         info!("{:?}", self.props.difficulty);
//         if depth >= max_depth {
//             // if slow (or memory consumption is high), lower the value
//             let mut ret_val = 0;

//             // if win, value = +inf
//             let win_val = val.0;
//             let chain_val = val.1 * ai_move_value;
//             ret_val = chain_val;

//             // If it lead to winning, then do it
//             if win_val == 4 * ai_move_value {
//                 // AI win, AI wants to win of course
//                 ret_val = 999999;
//             } else if win_val == 4 * ai_move_value * -1 {
//                 // AI lose, AI hates losing
//                 ret_val = 999999 * -1;
//             }
//             ret_val -= depth * depth;

//             return (ret_val, -1);
//         }

//         let win = val.0;
//         // if already won, then return the value right away
//         if win == 4 * ai_move_value {
//             // AI win, AI wants to win of course
//             return (999999 - depth * depth, -1);
//         }
//         if win == 4 * ai_move_value * -1 {
//             // AI lose, AI hates losing
//             return (999999 * -1 - depth * depth, -1);
//         }

//         if depth % 2 == 0 {
//             return self.min_state(ai_move_value, state, depth + 1, alpha, beta);
//         }
//         return self.max_state(ai_move_value, state, depth + 1, alpha, beta);
//     }

//     pub fn max_state(
//         &self,
//         ai_move_value: i64,
//         state: &Vec<Vec<i64>>,
//         depth: i64,
//         mut alpha: i64,
//         mut beta: i64,
//     ) -> (i64, i64) {
//         let mut v = -100000000007;
//         let mut new_move: i64 = -1;
//         let mut move_queue = Vec::new();

//         for j in 0..7 {
//             let temp_state = self.fill_map(state, j, ai_move_value);
//             if temp_state[0][0] != 999 {
//                 let temp_val = self.value(ai_move_value, &temp_state, depth, alpha, beta);
//                 if temp_val.0 > v {
//                     v = temp_val.0;
//                     new_move = j as i64;
//                     move_queue = Vec::new();
//                     move_queue.push(j);
//                 } else if temp_val.0 == v {
//                     move_queue.push(j);
//                 }

//                 // alpha-beta pruning
//                 if v > beta {
//                     new_move = self.choose(&move_queue);
//                     return (v, new_move);
//                 }
//                 alpha = std::cmp::max(alpha, v);
//             }
//         }
//         new_move = self.choose(&move_queue);

//         return (v, new_move);
//     }

//     pub fn min_state(
//         &self,
//         ai_move_value: i64,
//         state: &Vec<Vec<i64>>,
//         depth: i64,
//         mut alpha: i64,
//         mut beta: i64,
//     ) -> (i64, i64) {
//         let mut v = 100000000007;
//         let mut new_move: i64 = -1;
//         let mut move_queue = Vec::new();

//         for j in 0..7 {
//             let temp_state = self.fill_map(state, j, ai_move_value * -1);
//             if temp_state[0][0] != 999 {
//                 let temp_val = self.value(ai_move_value, &temp_state, depth, alpha, beta);
//                 if temp_val.0 < v {
//                     v = temp_val.0;
//                     new_move = j as i64;
//                     move_queue = Vec::new();
//                     move_queue.push(j);
//                 } else if temp_val.0 == v {
//                     move_queue.push(j);
//                 }

//                 // alpha-beta pruning
//                 if v < alpha {
//                     new_move = self.choose(&move_queue);
//                     return (v, new_move);
//                 }
//                 beta = std::cmp::min(beta, v);
//             }
//         }
//         new_move = self.choose(&move_queue);

//         return (v, new_move);
//     }

//     #[inline]
//     pub fn get_random_val(&self, val: usize) -> usize {
//         let rand = js! { return Math.random(); };
//         let base: f64 = rand.try_into().unwrap();
//         let max_val = val as f64;

//         return (base * max_val).floor() as usize;
//     }

//     #[inline]
//     pub fn choose(&self, choice: &Vec<usize>) -> i64 {
//         let index = self.get_random_val(choice.len());
//         return choice[index] as i64;
//     }

//     pub fn ai(&mut self, ai_move_value: i64) {
//         let new_map = self.map.clone();
//         let val_choice = self.max_state(ai_move_value, &new_map, 0, -100000000007, 100000000007);

//         let val = val_choice.0;
//         let choice = val_choice.1;

//         self.paused = false;
//         // TODO: Add rejectclick callback
//         let mut done = self.player_action(choice as usize, true);

//         // TODO: Add rejectclick callback
//         while done < 0 {
//             log::info!("Using random agent");
//             let random_choice = self.get_random_val(7);
//             done = self.player_action(random_choice, true);
//         }
//     }

//     pub fn fill_map(&self, new_state: &Vec<Vec<i64>>, column: usize, value: i64) -> Vec<Vec<i64>> {
//         let mut temp_map = new_state.clone();
//         if temp_map[0][column] != 0 || column > 6 {
//             temp_map[0][0] = 999; // error code
//         }

//         let mut done = false;
//         let mut row = 0;

//         for i in 0..5 {
//             if temp_map[i + 1][column] != 0 {
//                 done = true;
//                 row = i;
//                 break;
//             }
//         }
//         if !done {
//             row = 5;
//         }

//         temp_map[row][column] = value;
//         return temp_map;
//     }

//     pub fn draw_circle(&self, x: u32, y: u32, fill: &str, stroke: &str, text: &str) {
//         self.canvas_context.as_ref().unwrap().save();
//         self.canvas_context.as_ref().unwrap().set_fill_style_color(&fill);
//         self.canvas_context.as_ref().unwrap().set_stroke_style_color(&stroke);
//         self.canvas_context.as_ref().unwrap().begin_path();
//         self.canvas_context
//             .as_ref()
//             .unwrap()
//             .arc(x as f64, y as f64, 25.0, 0.0, 2.0 * 3.14159265359, false);
//         self.canvas_context.as_ref().unwrap().fill(FillRule::NonZero);
//         self.canvas_context.as_ref().unwrap().restore();

//         let context = self.canvas_context.as_ref().unwrap();
//         context.set_font("bold 30px serif");
//         context.restore();
//         context.fill_text(text, x as f64 - 12.0, y as f64 + 12.0, None);
//     }

//     pub fn draw_mask(&self) {
//         self.canvas_context.as_ref().unwrap().save();
//         self.canvas_context.as_ref().unwrap().set_fill_style_color("#00bfff");
//         self.canvas_context.as_ref().unwrap().begin_path();
//         for y in 0..6 {
//             for x in 0..7 {
//                 self.canvas_context.as_ref().unwrap().arc(
//                     (75 * x + 100) as f64,
//                     (75 * y + 50) as f64,
//                     25.0,
//                     0.0,
//                     2.0 * 3.14159265359,
//                     false,
//                 );
//                 self.canvas_context.as_ref().unwrap().rect(
//                     (75 * x + 150) as f64,
//                     (75 * y) as f64,
//                     -100.0,
//                     100.0,
//                 );
//             }
//         }
//         self.canvas_context.as_ref().unwrap().fill(FillRule::NonZero);
//         self.canvas_context.as_ref().unwrap().restore();
//     }

//     pub fn draw(&self) {
//         for y in 0..6 {
//             for x in 0..7 {
//                 let mut fg_color = "transparent";
//                 if self.map[y][x] >= 1 {
//                     fg_color = "#ff4136";
//                 } else if self.map[y][x] <= -1 {
//                     fg_color = "#ffff00";
//                 }
//                 self.draw_circle(
//                     (75 * x + 100) as u32,
//                     (75 * y + 50) as u32,
//                     &fg_color,
//                     "black",
//                     if self.map[y][x] >= 1 {
//                         "X"
//                     } else if self.map[y][x] <= -1 {
//                         "O"
//                     } else {
//                         ""
//                     },
//                 );
//             }
//         }
//     }

//     pub fn check(&mut self) {
//         let mut right = 0;
//         let mut down = 0;
//         let mut down_right = 0;
//         let mut up_right = 0;

//         for i in 0..6 {
//             for j in 0..7 {
//                 right = 0;
//                 down = 0;
//                 down_right = 0;
//                 up_right = 0;
//                 for k in 0..4 {
//                     if j + k < 7 {
//                         right += self.map[i][j + k];
//                     }

//                     if i + k < 6 {
//                         down += self.map[i + k][j];
//                     }

//                     if i + k < 6 && j + k < 7 {
//                         down_right += self.map[i + k][j + k];
//                     }

//                     if i >= k && j + k < 7 {
//                         up_right += self.map[i - k][j + k];
//                     }
//                 }

//                 if right.abs() == 4 {
//                     self.win(right);
//                 } 
//                 else if down.abs() == 4 {
//                     self.win(down);
//                 } 
//                 else if down_right.abs() == 4 {
//                     self.win(down_right);
//                 } 
//                 else if up_right.abs() == 4 {
//                     self.win(up_right);
//                 }
//             }
//         }

//         // check if draw
//         if (self.current_turn == 42) && (!self.won) {
//             self.win(0);
//         }
//     }

//     pub fn clear(&self) {
//         self.canvas_context.as_ref().unwrap().clear_rect(
//             0.0,
//             0.0,
//             self.canvas.as_ref().unwrap().width() as f64,
//             self.canvas.as_ref().unwrap().height() as f64,
//         );
//     }

//     pub fn on_region(&self, coord: f64, x: f64, radius: f64) -> bool {
//         return ((coord - x) * (coord - x) <= radius * radius);
//     }

//     pub fn player_move(&self) -> i64 {
//         if self.current_turn % 2 == 0 {
//             return 1;
//         }
//         return -1;
//     }

//     pub fn animate(
//         &mut self,
//         column: usize,
//         current_turn: i64,
//         to_row: usize,
//         cur_pos: usize,
//         mode: bool,
//     ) {
//         let mut fg_color = "transparent";
//         if current_turn >= 1 {
//             fg_color = "#ff4136";
//         } else if current_turn <= -1 {
//             fg_color = "#ffff00";
//         }

//         if to_row * 75 >= cur_pos {
//             self.clear();
//             self.draw();
//             self.draw_circle(
//                 (75 * column + 100) as u32,
//                 (cur_pos + 50) as u32,
//                 &fg_color,
//                 "black",
//                 if self.player_move() == 1 { "X" } else { "O" },
//             );
//             self.draw_mask();

//             let cloned = self.animate_call_back_click.clone();
//             window().request_animation_frame(enclose!((cloned) move |_| {
//                 cloned.emit((column, current_turn, to_row, cur_pos+25, mode));
//             }));
//         } else {
//             self.map[to_row][column] = self.player_move();
//             self.current_turn += 1;
//             self.draw();
//             self.check();
//             if mode == false && self.props.player2.as_ref().unwrap() == "Computer" {
//                 self.ai(-1);
//             } else {
//                 self.reject_click = false;
//             }
//         }
//     }

//     pub fn player_action(&mut self, column: usize, mode: bool) -> i64 {
//         if self.paused || self.won {
//             return 0;
//         }

//         if self.map[0][column] != 0 || column > 6 {
//             return -1;
//         }

//         let mut done = false;
//         let mut row = 0;
//         for i in 0..5 {
//             if self.map[i + 1][column] != 0 {
//                 done = true;
//                 row = i;
//                 break;
//             }
//         }
//         if !done {
//             row = 5;
//         }

//         self.animate(column, self.player_move(), row, 0, mode);

//         self.paused = true;
//         return 1;
//     }

//     pub fn win(&mut self, player: i64) {
//         self.paused = true;
//         self.won = true;
//         self.reject_click = false;

//         let mut msg = String::new();
//         if player > 0 {
//             msg = format!("{} wins", self.props.player1.as_ref().unwrap());
//         } else if player < 0 {
//             msg = format!("{} wins", self.props.player2.as_ref().unwrap());
//         } else {
//             msg = "It's a draw".to_string();
//         }

//         let print_msg = format!("{} - Click on game board to reset", msg);

//         self.canvas_context.as_ref().unwrap().save();
//         self.canvas_context.as_ref().unwrap().set_font("14pt sans-serif");
//         self.canvas_context.as_ref().unwrap().set_fill_style_color("#111");
//         self.canvas_context
//             .as_ref()
//             .unwrap()
//             .fill_text(&print_msg, 150.0, 20.0, None);

//         // construct game to post
//         let game = Game {
//             gameNumber: String::new(),
//             gameType: String::from("Connect-4"),
//             Player1Name: self.props.player1.as_ref().unwrap().clone(),
//             Player2Name: self.props.player2.as_ref().unwrap().clone(),
//             WinnerName: if player > 0 {
//                 self.props.player1.as_ref().unwrap().clone()
//             } else if player < 0 {
//                 self.props.player2.as_ref().unwrap().clone()
//             } else {
//                 String::from("Draw")
//             },
//             GameDate: Date::now() as u64,
//         };

//         // construct callback
//         let callback = self
//             .link
//             .callback(move |response: Response<Result<String, Error>>| {
//                 info!("successfully saved!");
//                 Message::Ignore
//             });

//         // construct request
//         let request = Request::post("/games")
//             .header("Content-Type", "application/json")
//             .body(Json(&game))
//             .unwrap();

//         // send the request
//         self.fetch_task = self.fetch_service.fetch(request, callback).ok();

//         self.canvas_context.as_ref().unwrap().restore();
//     }
// }

// impl Component for CanvasModel {
//     type Message = Message;
//     type Properties = Props;

//     fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
//         let canvas_id = props.canvas_id.clone().unwrap();

//         let mut map: Vec<Vec<i64>> = vec![vec![0; 7]; 6];

//         Self {
//             props,
//             canvas_id,
//             canvas: None,
//             canvas_context: None,
//             call_back_click: link.callback(|e: ClickEvent| Message::Click(e)),
//             animate_call_back_click: link
//                 .callback(|e: (usize, i64, usize, usize, bool)| Message::AnimateCallback(e)),
//             map,
//             current_turn: 0,
//             paused: false,
//             won: false,
//             reject_click: false,
//             fetch_service: FetchService::new(),
//             fetch_task: None,
//             link,
//         }
//     }

//     fn update(&mut self, message: Self::Message) -> ShouldRender {
//         match message {
//             Message::Click(e) => {
//                 if self.reject_click {
//                     return false;
//                 }

//                 if self.won {
//                     self.reset();
//                     self.props.game_done_call_back_click.emit(0);
//                     return true;
//                 }

//                 let rect = self.canvas.as_ref().unwrap().get_bounding_client_rect();
//                 let x = e.client_x() as f64 - rect.get_left();

//                 for j in 0..7 {
//                     if self.on_region(x, (75 * j + 100) as f64, 25 as f64) {
//                         self.paused = false;

//                         let valid = self.player_action(j, false);
//                         if valid == 1 {
//                             self.reject_click = true;
//                         };

//                         break;
//                     }
//                 }
//             }
//             Message::AnimateCallback((a, b, c, d, e)) => {
//                 self.animate(a, b, c, d, e);
//             }
//             Message::Ignore => {}
//         };

//         true
//     }

//     fn view(&self) -> Html {
//         html! {
//             <canvas id={&self.canvas_id} height="480" width="640"></canvas>
//         }
//     }

//     fn mounted(&mut self) -> ShouldRender {
//         self.canvas = Some(canvas(self.canvas_id.as_str()));
//         self.canvas_context = Some(context(self.canvas_id.as_str()));

//         let canvas_context = self.canvas_context.as_ref().unwrap();
//         let cloned_call_back_click = self.call_back_click.clone();

//         self.canvas.as_ref().unwrap().add_event_listener(enclose!(
//             (canvas_context) move | event: ClickEvent | {
//                 cloned_call_back_click.emit(event);
//             }
//         ));

//         // clears and draws mask
//         self.reset();

//         true
//     }

//     fn change(&mut self, props: Self::Properties) -> ShouldRender {
//         self.props = props;
//         true
//     }
// }

// #[inline(always)]
// fn canvas(id: &str) -> CanvasElement {
//     document()
//         .query_selector(&format!("#{}", id))
//         .unwrap()
//         .expect(&format!("Failed to select canvas id #{}", id))
//         .try_into()
//         .unwrap()
// }

// #[inline(always)]
// fn context(id: &str) -> CanvasRenderingContext2d {
//     canvas(id).get_context().unwrap()
// }

use std::error::Error;
use stdweb::traits::*;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::Date;
use stdweb::web::FillRule;
use stdweb::web::{document, window, CanvasRenderingContext2d};
use stdweb::web::event::ClickEvent;
// use yew::format::Json;
// use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{prelude::*, virtual_dom::VNode, Properties};
use log;
use crate::connect4Computer::Difficulty::{self, *};
use crate::ScoreBoard::Game;

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

pub struct CanvasModel {
    props: Props,
    canvas_id: String,
    canvas: Option<CanvasElement>,
    canvas_context: Option<CanvasRenderingContext2d>,
    call_back_click: Callback<ClickEvent>,
    animate_call_back_click: Callback<(usize, i64, usize, usize, bool)>,
    map: Vec<Vec<i64>>,
    current_turn: i64,
    won: bool,
    paused: bool,
    reject_click: bool,
    // fetch_service: FetchService,
    // fetch_task: Option<FetchTask>,
    // link: ComponentLink<CanvasModel>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub player1: Option<String>,
    pub player2: Option<String>,
    pub difficulty: Difficulty,
    pub canvas_id: Option<String>,
    pub game_done_call_back_click: Callback<i64>,
}

pub enum Message {
    Click(ClickEvent),
    AnimateCallback((usize, i64, usize, usize, bool)),
    Ignore,
}

impl CanvasModel {
    pub fn reset(&mut self) {
        self.map = vec![vec![0; 7]; 6];
        self.current_turn = 0;
        self.paused = false;
        self.won = false;
        self.reject_click = false;
        self.clear();
        self.draw_mask();
    }

    pub fn add_disk(&self, old_board: &Vec<Vec<i64>>, column: usize, value: i64) -> Vec<Vec<i64>> {
        let mut new_board = old_board.clone();
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
    }
    
    pub fn check(&mut self) {
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
                        right += self.map[i][j + k];
                    }
    
                    if i + k < 6 {
                        down += self.map[i + k][j];
                    }
    
                    if i + k < 6 && j + k < 7 {
                        down_right += self.map[i + k][j + k];
                    }
    
                    if i >= k && j + k < 7 {
                        up_right += self.map[i - k][j + k];
                    }
                }
    
                if right.abs() == 4 {
                    self.win(right);
                } 
                else if down.abs() == 4 {
                    self.win(down);
                } 
                else if down_right.abs() == 4 {
                    self.win(down_right);
                } 
                else if up_right.abs() == 4 {
                    self.win(up_right);
                }
            }
        }
        
        // check if the game is a tie
        if (self.current_turn == 42) && (!self.won) {
            self.win(0);
        }
    }

    pub fn player_action(&mut self, column: usize, mode: bool) -> i64 {
        if self.paused || self.won {
            return 0;
        }

        if self.map[0][column] != 0 || column > 6 {
            return -1;
        }

        let mut done = false;
        let mut row = 0;
        for i in (0..6).rev() {
            if self.map[i][column] == 0 {
                done = true;
                row = i;
                break;
            }
        }

        self.animate(column, self.player_move(), row, 0, mode);

        self.paused = true;
        return 1;
    }

    pub fn player_move(&self) -> i64 {
        if self.current_turn % 2 == 0 {
            return 1;
        }
        return -1;
    }

    pub fn ai(&mut self, ai_move_value: i64) {
        let new_map = self.map.clone();
        let val_choice = self.max_state(ai_move_value, &new_map, 0, -100000000007, 100000000007);

        let val = val_choice.0;
        let choice = val_choice.1;
        self.paused = false;
        let mut done = self.player_action(choice as usize, true);
        
        while done < 0 {
            log::info!("Using random agent");
            let random_choice = self.get_random_val(7);
            done = self.player_action(random_choice, true);
        }
    }

    pub fn win(&mut self, winner: i64) {
        self.paused = true;
        self.won = true;
        self.reject_click = false;

        let mut msg = String::new();
        if winner > 0 {
            msg = format!("{} wins", self.props.player1.as_ref().unwrap());
        } else if winner < 0 {
            msg = format!("{} wins", self.props.player2.as_ref().unwrap());
        } else {
            msg = "It's a draw".to_string();
        }

        let print_msg = format!("{} - Click on game board to reset", msg);

        self.canvas_context.as_ref().unwrap().save();
        self.canvas_context.as_ref().unwrap().set_fill_style_color("#000");
        self.canvas_context.as_ref().unwrap().fill_text(&print_msg, 150.0, 20.0, None);

        let game = Game {
            gameNumber: String::new(),
            gameType: String::from("Connect-4"),
            Player1Name: self.props.player1.as_ref().unwrap().clone(),
            Player2Name: self.props.player2.as_ref().unwrap().clone(),
            WinnerName: if winner > 0 {
                self.props.player1.as_ref().unwrap().clone()
            } else if winner < 0 {
                self.props.player2.as_ref().unwrap().clone()
            } else {
                String::from("Draw")
            },
            GameDate: Date::now() as u64,
        };

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

        self.canvas_context.as_ref().unwrap().restore();
    }

    pub fn draw_circle(&self, x: u32, y: u32, color: &str, outline: &str) {
        self.canvas_context.as_ref().unwrap().save();
        self.canvas_context.as_ref().unwrap().set_fill_style_color(&color);
        self.canvas_context.as_ref().unwrap().set_stroke_style_color(&outline);
        self.canvas_context.as_ref().unwrap().begin_path();
        self.canvas_context.as_ref().unwrap().arc(x as f64, y as f64, 25.0, 0.0, 2.0 * std::f64::consts::PI, false);
        self.canvas_context.as_ref().unwrap().fill(FillRule::NonZero);
        self.canvas_context.as_ref().unwrap().restore();
    }

    pub fn draw_mask(&self) {
        self.canvas_context.as_ref().unwrap().save();
        self.canvas_context.as_ref().unwrap().set_fill_style_color("#00bfff");
        self.canvas_context.as_ref().unwrap().begin_path();
        for y in 0..6 {
            for x in 0..7 {
                self.canvas_context.as_ref().unwrap().arc((75 * x + 100) as f64, (75 * y + 50) as f64, 25.0, 0.0, 2.0 * std::f64::consts::PI, false);
                self.canvas_context.as_ref().unwrap().rect((75 * x + 150) as f64, (75 * y) as f64, -100.0, 100.0);
            }
        }
        self.canvas_context.as_ref().unwrap().fill(FillRule::NonZero);
        self.canvas_context.as_ref().unwrap().restore();
    }

    pub fn draw(&self) {
        for y in 0..6 {
            for x in 0..7 {
                let mut fg_color = "transparent";
                if self.map[y][x] >= 1 {
                    fg_color = "#ff0000";
                } else if self.map[y][x] <= -1 {
                    fg_color = "#ffff00";
                }
                self.draw_circle((75 * x + 100) as u32, (75 * y + 50) as u32, &fg_color, "black");
            }
        }
    }

    pub fn animate(&mut self, column: usize, current_turn: i64, to_row: usize, cur_pos: usize, mode: bool) {
        let mut fg_color = "transparent";
        if current_turn >= 1 {
            fg_color = "#ff0000";
        } else if current_turn <= -1 {
            fg_color = "#ffff00";
        }

        if to_row * 75 >= cur_pos {
            self.clear();
            self.draw();
            self.draw_circle((75 * column + 100) as u32, (cur_pos + 50) as u32, &fg_color, "black");
            self.draw_mask();

            let cloned = self.animate_call_back_click.clone();
            window().request_animation_frame(enclose!((cloned) move |_| {
                cloned.emit((column, current_turn, to_row, cur_pos+25, mode));
            }));
        } else {
            self.map[to_row][column] = self.player_move();
            self.current_turn += 1;
            self.draw();
            self.check();
            if mode == false && self.props.player2.as_ref().unwrap() == "Computer" {
                self.ai(-1);
            } else {
                self.reject_click = false;
            }
        }
    }

    pub fn clear(&self) {
        self.canvas_context.as_ref().unwrap().clear_rect(0.0, 0.0, self.canvas.as_ref().unwrap().width() as f64, self.canvas.as_ref().unwrap().height() as f64);
    }

}

// impl Component for CanvasModel {
//     type Message = Message;
//     type Properties = Props;

//     fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
//         let canvas_id = props.canvas_id.clone().unwrap();

//         let mut map: Vec<Vec<i64>> = vec![vec![0; 7]; 6];

//         Self {
//             props,
//             canvas_id,
//             canvas: None,
//             canvas_context: None,
//             call_back_click: link.callback(|e: ClickEvent| Message::Click(e)),
//             animate_call_back_click: link
//                 .callback(|e: (usize, i64, usize, usize, bool)| Message::AnimateCallback(e)),
//             map,
//             current_turn: 0,
//             paused: false,
//             won: false,
//             reject_click: false,
//             fetch_service: FetchService::new(),
//             fetch_task: None,
//             link,
//         }
//     }

//     fn update(&mut self, message: Self::Message) -> ShouldRender {
//         match message {
//             Message::Click(e) => {
//                 if self.reject_click {
//                     return false;
//                 }

//                 if self.won {
//                     self.reset();
//                     self.props.game_done_call_back_click.emit(0);
//                     return true;
//                 }

//                 let rect = self.canvas.as_ref().unwrap().get_bounding_client_rect();
//                 let x = e.client_x() as f64 - rect.get_left();

//                 for j in 0..7 {
//                     if self.on_region(x, (75 * j + 100) as f64, 25 as f64) {
//                         self.paused = false;

//                         let valid = self.player_action(j, false);
//                         if valid == 1 {
//                             self.reject_click = true;
//                         };

//                         break;
//                     }
//                 }
//             }
//             Message::AnimateCallback((a, b, c, d, e)) => {
//                 self.animate(a, b, c, d, e);
//             }
//             Message::Ignore => {}
//         };

//         true
//     }

//     fn view(&self) -> Html {
//         html! {
//             <canvas id={&self.canvas_id} height="480" width="640"></canvas>
//         }
//     }

//     fn mounted(&mut self) -> ShouldRender {
//         self.canvas = Some(canvas(self.canvas_id.as_str()));
//         self.canvas_context = Some(context(self.canvas_id.as_str()));

//         let canvas_context = self.canvas_context.as_ref().unwrap();
//         let cloned_call_back_click = self.call_back_click.clone();

//         self.canvas.as_ref().unwrap().add_event_listener(enclose!(
//             (canvas_context) move | event: ClickEvent | {
//                 cloned_call_back_click.emit(event);
//             }
//         ));

//         // clears and draws mask
//         self.reset();

//         true
//     }

//     fn change(&mut self, props: Self::Properties) -> ShouldRender {
//         self.props = props;
//         true
//     }
// }

