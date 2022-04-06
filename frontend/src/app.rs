use yew::prelude::*;
use yew_router::prelude::*;
use crate::home::Home;

#[derive(Clone, Routable, PartialEq, Debug)]
enum AppRoute {
    #[at("/")]
    Home,
    #[at("/connect4")]
    Connect4,
    #[not_found]
    #[at("/404")]
    NotFound,
}
pub enum Msg { }
pub struct App {
}

fn switch(route: &AppRoute) -> Html {
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

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        // match msg { }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <BrowserRouter>
                <div>
                    <div class="header">
                        <Link<AppRoute> classes={classes!("header-logo-link")} to={AppRoute::Home}>
                            <img class="header-logo" src="img/rust.png" />
                            <p class="header-logo-text">{ "Rusty Connect 4" }</p>
                        </Link<AppRoute>>
                        <div class="header-pages">
                            <Link<AppRoute> classes={classes!("header-page-link")} to={AppRoute::Home}>
                                <p class="header-page-text">{ "Home" }</p>
                            </Link<AppRoute>>
                            <Link<AppRoute> classes={classes!("header-page-link")} to={AppRoute::Connect4}>
                                <p class="header-page-text">{ "Connect4" }</p>
                            </Link<AppRoute>>
                        </div>
                    </div>
                    <Switch<AppRoute> render={Switch::render(switch)} />
                </div>
            </BrowserRouter>
        }
    }
}