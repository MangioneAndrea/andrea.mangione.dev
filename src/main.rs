use leptos::prelude::*;

mod components;

use components::topbar::Topbar;

fn main() {
    leptos::mount::mount_to_body(App)
}


#[component]
fn app() -> impl IntoView {
    view! {
        <Topbar />
    }
}
