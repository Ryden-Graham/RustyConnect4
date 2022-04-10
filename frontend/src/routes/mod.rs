use yew::prelude::*;
use yew_router::prelude::*;

use crate::home::Home;

use crate::howToConnect4::HowToConnect4;
use crate::connect4Computer::Connect4Computer;
use crate::connect4Human::Connect4Human;

use crate::howToTootOtto::HowToTootOtto;
use crate::tootOttoComputer::TootOttoComputer;
use crate::tootOttoHuman::TootOttoHuman;

use crate::history::History;
use crate::scores::Scores;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum AppRoute {
    #[at("/")]
    Home,

    #[at("/HowToConnect4")]
    HowToConnect4,
    #[at("/Connect4Computer")]
    Connect4Computer,
    #[at("/Connect4Human")]
    Connect4Human,

    #[at("/HowToTootOtto")]
    HowToTootOtto,
    #[at("/TootOttoComputer")]
    TootOttoComputer,
    #[at("/TootOttoHuman")]
    TootOttoHuman,

    #[at("/History")]
    History,
    #[at("/Scores")]
    Scores,
    #[not_found]
    #[at("/404")]
    NotFound
}

pub fn switch(route: &AppRoute) -> Html {
    match route {
        AppRoute::Home => html! { <Home />},

        AppRoute::HowToConnect4 => html! { <HowToConnect4 /> },
        AppRoute::Connect4Computer => html! { <Connect4Computer /> },
        AppRoute::Connect4Human => html! { <Connect4Human /> },

        AppRoute::HowToTootOtto => html! { <HowToTootOtto /> },
        AppRoute::TootOttoComputer => html! { <TootOttoComputer /> },
        AppRoute::TootOttoHuman => html! { <TootOttoHuman /> },

        AppRoute::History => html! { <History />},
        AppRoute::Scores => html! { <Scores />},
        AppRoute::NotFound => html! {
            <h1>{"404"}</h1>
        },
    }
}