mod about;
mod stack;

use leptos::{component, view, IntoView};

use about::About;
use stack::Stack;

use crate::components::stack_visualizer::StackVisualizer;

#[component()]
pub fn home() -> impl IntoView {
    view! { 
        <About/>
        <Stack/>

        <StackVisualizer/>

    }
}
