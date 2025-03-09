use leptos::prelude::*;

mod components;
mod routes;

use components::topbar::Topbar;
use leptos_router::components::Router;
use routes::RoutesSlot;

fn main() {
    leptos::mount::mount_to_body(App)
}

#[component]
fn app() -> impl IntoView {
    view! {
        <Router>
            <Topbar />
            <RoutesSlot/>
        </Router>
    }
}
