use leptos::{prelude::{set_interval, signal, CollectView, Get, Set, StyleAttribute, Update}, svg::Circle, task::spawn_local, *};
use leptos::prelude::ElementChild;
use leptos::prelude::CustomAttribute;
use leptos::prelude::ClassAttribute;
use std::{f64::{self, consts::PI}, time::Duration};
use web_sys::window;

#[derive(Default)]
struct Point {
    min_x: f64,
    min_y: f64,
    x: f64,
    y: f64,
    max_x: f64,
    max_y: f64,
    t_x: f64,
    t_y: f64,
    default_extrusion_x: f64,
    default_extrusion_y: f64,
}


struct Language {
    image: &'static str,
    key: &'static str
}

#[component]
pub fn stack_visualizer() -> impl IntoView {
    let (x, set_x) = signal(1);

    set_interval(move ||{
    }, Duration::from_millis(1));

    // Return view with Leptos components
    view! { 
        <div class="bg-white w-1 h-1" style=move||format!("width: {}px;", x.get())>
        </div>
    }
}
