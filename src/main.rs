use leptos::prelude::*;

mod components;
mod routes;

use components::topbar::Topbar;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use routes::{Routes as R, A, B};

fn main() {
    leptos::mount::mount_to_body(App)
}

#[component]
fn app() -> impl IntoView {
    view! {
        <Router>
            <Topbar />

            <Routes fallback=|| "oo">
                <Route path=path!("") view=A />
                <Route path=path!("/blog") view=B />
            </Routes>

        </Router>
    }
}
