use yew::prelude::*;
use yew_router::prelude::*;

use crate::howToConnect4::HowToConnect4;
use crate::connect4Computer::Connect4Computer;
use crate::howToToot::HowToToot;
use crate::home::Home;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum AppRoute {
    #[at("/HowToConnect4")]
    HowToConnect4,
    #[at("/Connect4Computer")]
    Connect4Computer,
    #[at("/HowToToot")]
    HowToToot,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &AppRoute) -> Html {
    match route {
        AppRoute::Home => html! { <Home />},
        AppRoute::HowToConnect4 => html! { <HowToConnect4 /> },
        AppRoute::Connect4Computer => html! { <Connect4Computer /> },
        AppRoute::HowToToot => html! { <HowToToot /> },
        AppRoute::NotFound => html! {
            <h1>{"404"}</h1>
        },
    }
}