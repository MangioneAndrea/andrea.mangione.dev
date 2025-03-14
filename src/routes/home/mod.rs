mod about;
mod stack;
mod projects;

use leptos::{component, view, IntoView};

use about::About;
use stack::Stack;
use projects::Projects;


#[component()]
pub fn home() -> impl IntoView {
    view! {
        <About />
        <Stack />
        <Projects />
    }
}
