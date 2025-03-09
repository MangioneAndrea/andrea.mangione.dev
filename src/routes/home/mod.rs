mod about;
mod stack;

use leptos::{component, view, IntoView};

use about::About;
use stack::Stack;

#[component()]
pub fn home() -> impl IntoView {
    view! { 
        <About/>
        <Stack/>


    }
}
