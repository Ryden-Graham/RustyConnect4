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
pub enum Msg {
    SwitchHome,
}
pub struct Header;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub title: String,
}


impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SwitchHome => {
                html! {
                    <h1>{ "Home" }</h1>
                };
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        html! {
            <div class="header">
                <div class="header-logo-link" onclick={ctx.link().callback(|_| Msg::SwitchHome)}>
                    <img class="header-logo" src="img/rust.png" />
                    <p class="header-logo-text">{ ctx.props().title.clone() }</p>
                </div>
                <div class="header-pages">
                    <a class="header-page-link" href = "/">
                        <p class="header-page-text">{ "Home" }</p>
                    </a>
                    <a class="header-page-link" href = "/connect4">
                        <p class="header-page-text">{ "Connect4" }</p>
                    </a>
                </div>
            </div>
        }
    }
}

// use std::borrow::Borrow;
// use yew_router::prelude::*;

// use yew::prelude::*;

// pub enum Msg { 
//     SwitchHome,
//     SwitchConnect4,
// }
// pub struct Header;

// #[derive(Clone, Routable, PartialEq)]
// enum AppRoute {
//     #[at("/")]
//     Home,
//     #[at("/connect4")]
//     Connect4,
//     #[not_found]
//     #[at("/404")]
//     NotFound,
// }

// #[derive(Clone, PartialEq, Properties)]
// pub struct Props {
//     #[prop_or_default]
//     pub title: String,
// }

// impl Component for Header {
//     type Message = Msg;
//     type Properties = Props;

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self
//     }

//     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
//         // match msg { }
//         false
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         let history = ctx.link().history().unwrap(); // Unused atm but can be used for a back button
//         // let home_button = Callback::once(move |_| history.push(AppRoute::Home));
//         // let connect4_button = Callback::once(move |_| history.push(AppRoute::Home));
//         html! {
//             <div class="header">
//                 // <div class="header-logo-link" onclick={ctx.link().callback(|_| Msg::SwitchHome)}>
//                 //     <img class="header-logo" src="img/rust.png" />
//                 //     <p class="header-logo-text">{ ctx.props().title.clone() }</p>
//                 // </div>
//                 <div class="header-pages">
//                     <a class="header-page-link" href = "/">
//                         <p class="header-page-text">{ "Home" }</p>
//                     </a>
//                     <a class="header-page-link" href = "/connect4">
//                         <p class="header-page-text">{ "Connect4" }</p>
//                     </a>
//                 </div>
//             </div>
//         }
//     }
// }