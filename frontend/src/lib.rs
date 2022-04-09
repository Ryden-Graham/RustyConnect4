#![recursion_limit = "512"]

pub mod app;
pub mod routes;
pub mod home;
pub mod howToConnect4;
pub mod connect4Computer;
pub mod howToToot;
pub mod sidebar;
pub mod canvas;
pub mod ScoreBoard;

use wasm_bindgen::prelude::*;

use app::App;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
    Ok(())
}