use yew::prelude::*;
use yew_router::prelude::*;

use crate::home::Home;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum AppRoute {
    #[at("/connect4")]
    Connect4,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &AppRoute) -> Html {
    match route {
        AppRoute::Home => html! { <Home />},
        AppRoute::Connect4 => html! {
            <h1>{"Wowow look at our amazing connect 4 game!"}</h1>
        },
        AppRoute::NotFound => html! {
            <h1>{"404"}</h1>
        },
    }
}