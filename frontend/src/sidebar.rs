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
pub struct Sidebar;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub title: String,
}


impl Component for Sidebar {
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
            <>
                <nav class="w3-sidenav w3-red w3-collapse w3-top w3-large w3-padding" style="z-index:3;width:350px;font-weight:bold" id="mySidenav">
                    <a href="javascript:void(0)" class="w3-padding-xlarge w3-hide-large w3-display-topleft w3-hover-white" style="width:100%">{"Close Menu"}</a>
                    <div class="w3-container">
                        <h3 class="w3-padding-64"><b>{"Play"}<br/> {"Connect4 / TOOT-OTTO"}</b></h3>
                    </div>

                    // {for list_items}
                </nav>
                <header class="w3-container w3-top w3-hide-large w3-red w3-xlarge w3-padding">
                <a href="javascript:void(0)" class="w3-btn w3-red w3-border w3-border-white w3-margin-right">{"\u{2630}"}</a>
                <span>{"Connect 4 with MEAN"}</span>
                </header>
                <div class="w3-overlay w3-hide-large" style="cursor:pointer" title="close side menu" id="myOverlay"></div>
                <div class="w3-main" style="margin-left:390px;margin-right:40px">
                // {
                    // html !{
                    //     <DisplayWindow uri={active_markdown_uri} />
                    // }
                // }
                </div>
            </>
        }
    }
}

// <div class="header">
//     <Link<AppRoute> classes={classes!("header-logo-link")} to={AppRoute::Home}>
//         <img class="header-logo" src="img/rust.png" />
//         <p class="header-logo-text">{ "Rusty Connect 4" }</p>
//     </Link<AppRoute>>
//     <div class="header-pages">
//         <Link<AppRoute> classes={classes!("header-page-link")} to={AppRoute::Home}>
//             <p class="header-page-text">{ "Home" }</p>
//         </Link<AppRoute>>
//         <Link<AppRoute> classes={classes!("header-page-link")} to={AppRoute::Connect4}>
//             <p class="header-page-text">{ "Connect4" }</p>
//         </Link<AppRoute>>
//     </div>
// </div>

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