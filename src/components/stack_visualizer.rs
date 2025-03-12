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
            let x = IMAGE_RADIUS * angle.cos() + IMAGE_RADIUS * 2.;
            let y = IMAGE_RADIUS * angle.sin() + IMAGE_RADIUS * 2.;

            let ox = x / 2.; // visible confusion...
            let oy = y / 2.;

            (x, y, ox, oy, lan)
        })
        .collect();

    let (class, style) = stylers::style_str!{
        @keyframes rotating {
            0% {
                -ms-transform: rotate(0deg);
                -moz-transform: rotate(0deg);
                -webkit-transform: rotate(0deg);
                -o-transform: rotate(0deg);
                transform: rotate(-30deg);
            }

            50% {
                -ms-transform: rotate(360deg);
                -moz-transform: rotate(360deg);
                -webkit-transform: rotate(360deg);
                -o-transform: rotate(360deg);
                transform: rotate(30deg);
            }

            100% {
                -ms-transform: rotate(0deg);
                -moz-transform: rotate(0deg);
                -webkit-transform: rotate(0deg);
                -o-transform: rotate(0deg);
                transform: rotate(-20deg);
            }
        }

        :deep(.rotator) {
            animation: rotating 2s linear infinite;
        }
    };

    // Return view with Leptos components
    view! {
        class = class,
        <style>{style}</style>
        <svg width=200 height=200 class=class>
            {centers
                .into_iter()
                .map(|(x, y, ox, oy, lan)| {
                    view! {
                        <mask id=lan.key maskUnits="userSpaceOnUse">
                            <circle class="rotator" style=format!("transform-origin: {ox}px {oy}px;") fill="white" cx=IMAGE_RADIUS cy=IMAGE_RADIUS r=IMAGE_RADIUS />
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
