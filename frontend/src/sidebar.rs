use std::borrow::Borrow;

use yew_router::prelude::*;
use yew::prelude::*;
use crate::routes::AppRoute;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    html! {
        <div class="sidebar">
            <nav class="sidebar-container">
                <div class="sidebar-title">
                    <b>{"Play"}<br/> {"Connect4 / TOOT-OTTO"}</b>
                </div>
                <Link<AppRoute> classes={classes!("sidebar-page-link")} to={AppRoute::Home}>
                    <p class="header-page-text">{ "Home" }</p>
                </Link<AppRoute>>
                <br/>
                <Link<AppRoute> classes={classes!("sidebar-page-link")} to={AppRoute::HowToConnect4}>
                    <p class="header-page-text">{ "How to Play Connect4" }</p>
                </Link<AppRoute>>
                <Link<AppRoute> classes={classes!("sidebar-page-link")} to={AppRoute::Connect4Computer}>
                    <p class="header-page-text">{ "Play Connect4 With a Computer" }</p>
                </Link<AppRoute>>
                <Link<AppRoute> classes={classes!("sidebar-page-link")} to={AppRoute::Connect4Human}>
                    <p class="header-page-text">{ "Play Connect4 With a Human" }</p>
                </Link<AppRoute>>
                <br/>
                <Link<AppRoute> classes={classes!("sidebar-page-link")} to={AppRoute::HowToTootOtto}>
                    <p class="header-page-text">{ "How to Play TOOT-OTTO" }</p>
                </Link<AppRoute>>
                <Link<AppRoute> classes={classes!("sidebar-page-link")} to={AppRoute::TootOttoComputer}>
                    <p class="header-page-text">{ "Play TOOT-OTTO With a Computer" }</p>
                </Link<AppRoute>>
                <Link<AppRoute> classes={classes!("sidebar-page-link")} to={AppRoute::TootOttoHuman}>
                    <p class="header-page-text">{ "Play TOOT-OTTO With a Human" }</p>
                </Link<AppRoute>>
                <br/>
                <Link<AppRoute> classes={classes!("sidebar-page-link")} to={AppRoute::History}>
                    <p class="header-page-text">{ "View Game History" }</p>
                </Link<AppRoute>>
                <Link<AppRoute> classes={classes!("sidebar-page-link")} to={AppRoute::Scores}>
                    <p class="header-page-text">{ "Score Board" }</p>
                </Link<AppRoute>>
            </nav>
            <div class="sidebar-padder"/>
        </div>
    }
}
