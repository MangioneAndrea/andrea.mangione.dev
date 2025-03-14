use leptos::prelude::*;
use leptos::{component, view, IntoView};

#[component()]
pub fn projects() -> impl IntoView {
    view! {
        <h2 id="projects">
            <a href="#projects">Projects</a>
        </h2>
    }
}
