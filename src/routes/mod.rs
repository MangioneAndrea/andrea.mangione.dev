use leptos::{component, view, IntoView};
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq)]
pub enum Routes {
    Home,
    Blog,
}

impl Routes {
    pub const fn path(&self) -> &str {
        match self {
            Self::Home => "/",
            Self::Blog => "/blog",
        }
    }
}

#[component()]
pub fn a() -> impl IntoView {
    view! { "a" }
}
#[component()]
pub fn b() -> impl IntoView {
    view! { "b" }
}
