use leptos::prelude::ClassAttribute;
use leptos::prelude::CustomAttribute;
use leptos::prelude::ElementChild;
use leptos::{
    prelude::{set_interval, signal, CollectView, Get, Set, StyleAttribute, Update},
    svg::Circle,
    task::spawn_local,
    *,
};
use std::{
    f64::{self, consts::PI},
    time::Duration,
};
use web_sys::window;

#[derive(Default)]
struct Point {
    x: f64,
    y: f64,
}

struct Language {
    image: &'static str,
    key: &'static str,
}

const IMAGE_RADIUS: f64 = 50.;
#[component]
pub fn stack_visualizer() -> impl IntoView {
    let languages = vec![
        Language {
            image: "assets/Logos/Svelte.svg",
            key: "svelte",
        },
        Language {
            image: "assets/Logos/Solid.png",
            key: "solid",
        },
        Language {
            image: "assets/Logos/Astro.svg",
            key: "astro",
        },
    ];

    let sectors = languages.len();

    let radius = if sectors <= 1 { 0. } else { IMAGE_RADIUS };

    let centers: Vec<_> = languages
        .iter()
        .enumerate()
        .map(|(i, lan)| {
            // Calculate the angle for the current sector
            let angle = 2.0 * PI * (i as f64) / (sectors as f64);

            // Calculate the (x, y) position of the image
            let x = radius * angle.cos() + IMAGE_RADIUS * 2.;
            let y = radius * angle.sin() + IMAGE_RADIUS * 2.;

            (x, y, lan)
        })
        .collect();

    // Return view with Leptos components
    view! {
        <svg width=200 height=200>
            {centers
                .into_iter()
                .map(|(x, y, lan)| {
                    view! {
                        <mask id=lan.key x="0%" y="0%" maskUnits="userSpaceOnUse">
                            <circle fill="white" cx=IMAGE_RADIUS cy=IMAGE_RADIUS r=IMAGE_RADIUS />
                        </mask>
                        <g transform=format!(
                            "translate({},{})",
                            x - IMAGE_RADIUS,
                            y - IMAGE_RADIUS,
                        )>
                            <image
                                href=lan.image
                                height=IMAGE_RADIUS * 2.
                                mask=format!("url(#{})", lan.key)
                            />
                        </g>
                    }
                })
                .collect_view()}
        </svg>
    }
}
