use std::borrow::Borrow;

use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum AppRoute {
    #[at("/")]
    Home,
    #[at("/connect4")]
    Connect4,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    html! {
        <div class="sidebar">
            <nav class="sidebar-container">
                <div class="sidebar-title">
                // <div class="w3-container">
                    <b>{"Play"}<br/> {"Connect4 / TOOT-OTTO"}</b>
                </div>
                <Link<AppRoute> classes={classes!("sidebar-page-link")} to={AppRoute::Home}>
                    <p class="header-page-text">{ "Home" }</p>
                </Link<AppRoute>>
                <Link<AppRoute> classes={classes!("sidebar-page-link")} to={AppRoute::Connect4}>
                    <p class="header-page-text">{ "Connect4" }</p>
                </Link<AppRoute>>
            </nav>
            <div class="sidebar-padder"/>
        </div>
    }
}
