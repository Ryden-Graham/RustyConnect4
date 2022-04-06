use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{switch, AppRoute};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
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
        </BrowserRouter>
    }
}
