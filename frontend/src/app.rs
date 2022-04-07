use yew::prelude::*;
use yew_router::prelude::*;

use crate::sidebar::Sidebar;
use crate::routes::{switch, AppRoute};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="page-container">
                <Sidebar />
                <Switch<AppRoute> render={Switch::render(switch)} />
            </div>
        </BrowserRouter>
    }
}
