use leptos::prelude::*;
use leptos::{component, view, IntoView};

mod blog;
mod home;

use leptos_router::components::{Route, Routes};
use leptos_router::hooks::use_location;
use leptos_router::path;

use blog::Blog;
use home::Home;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq)]
pub enum Routes {
    Home,
    Blog,
}

impl Routes {
    pub const fn path(&self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Blog => "/blog",
        }
    }

    pub fn get_active() -> Self {
        let p = use_location().pathname.get();
        match p.as_str() {
            "/blog" => Self::Blog,
            _ => Self::Home,
        }
    }
}

#[component]
pub fn routes_slot() -> impl IntoView {
    view! {
        <div class="max-w-[60rem] mx-auto">
            <Routes fallback=|| "404">
                <Route path=path!("") view=Home />
                <Route path=path!("/blog") view=Blog />
            </Routes>
        </div>
    }
}
