use std::borrow::Borrow;

use yew::prelude::*;

pub enum Msg { }
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
        // match msg { }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        html! {
            <div class="header">
                <a class="header-logo-link" href = "/">
                    <img class="header-logo" src="img/rust.png" />
                    <p class="header-logo-text">{ ctx.props().title.clone() }</p>
                </a>
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