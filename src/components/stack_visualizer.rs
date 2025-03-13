use leptos::leptos_dom::logging::console_log;
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

#[derive(Default)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Language {
    image: &'static str,
    key: &'static str,
    color: &'static str,
}

const IMAGE_RADIUS: f64 = 100.;
#[component]
pub fn stack_visualizer() -> impl IntoView {
    let languages = vec![
        Language {
            image: "assets/Logos/Astro.svg",
            key: "astro",
            color: "white",
        },
        Language {
            image: "assets/Logos/Svelte.svg",
            key: "svelte",
            color: "red",
        },
        Language {
            image: "assets/Logos/Solid.png",
            key: "solid",
            color: "blue",
        },
    ];

    let sectors = languages.len();

    let centers: Vec<_> = languages
        .iter()
        .enumerate()
        .map(|(i, lan)| {
            let angle = 2.0 * PI * (i as f64) / (sectors as f64);

            let x = IMAGE_RADIUS * angle.cos() + IMAGE_RADIUS;
            let y = IMAGE_RADIUS * angle.sin() + IMAGE_RADIUS;

            let ox = 2. * IMAGE_RADIUS - x;
            let oy = 2. * IMAGE_RADIUS - y;

            (x, y, ox, oy, lan)
        })
        .collect();

    let (class, style) = stylers::style_str! {
        @keyframes rotating {
            0% {
                -ms-transform: rotate(-20deg);
                -moz-transform: rotate(-20deg);
                -webkit-transform: rotate(-20deg);
                -o-transform: rotate(-20deg);
                transform: rotate(-20deg);
            }

            50% {
                -ms-transform: rotate(20deg);
                -moz-transform: rotate(20deg);
                -webkit-transform: rotate(20deg);
                -o-transform: rotate(20deg);
                transform: rotate(20deg);
            }

            100% {
                -ms-transform: rotate(-20deg);
                -moz-transform: rotate(-20deg);
                -webkit-transform: rotate(-20deg);
                -o-transform: rotate(-20deg);
                transform: rotate(-20deg);
            }
        }

        :deep(.rotator) {
            animation: rotating 7s ease infinite;
        }
    };

    let v: Vec<_> = centers
        .iter()
        .cycle()
        .skip(1)
        .take(centers.len())
        .zip(centers.iter())
        .collect();

    // Return view with Leptos components
    view! { class=class,
        <style>{style}</style>
        <svg width=IMAGE_RADIUS * 4. height=IMAGE_RADIUS * 4. class=class>
            {v
                .into_iter()
                .map(|((x, y, ox, oy, lan), (nx, ny, nox, noy, nlan))| {
                    view! {
                        <mask id=lan.key>
                            <circle
                                class="rotator"
                                style=format!("transform-origin: {ox}px {oy}px;")
                                fill="black"
                                cx=IMAGE_RADIUS
                                cy=IMAGE_RADIUS
                                r=IMAGE_RADIUS
                            />
                            <circle
                                class="rotator"
                                style=format!("transform-origin: {ox}px {oy}px;")
                                fill="white"
                                cx=IMAGE_RADIUS
                                cy=IMAGE_RADIUS
                                r=IMAGE_RADIUS

                                stroke="black"
                                stroke-width="5px"
                            />
                            <circle
                                class="rotator"
                                style=format!("transform-origin: {ox}px {oy}px;")
                                fill="black"
                                cx=IMAGE_RADIUS - x + nx
                                cy=IMAGE_RADIUS - y + ny
                                r=IMAGE_RADIUS
                            />
                        </mask>
                        <g
                            transform=format!("translate({},{})", x, y)
                            mask=format!("url(#{})", lan.key)
                        >

                            <image
                                href=lan.image
                                height=IMAGE_RADIUS * 2.
                                width=IMAGE_RADIUS * 2.
                                // style=format!(
                                //     "transform: scale(0.6); transform-origin: {ox}px {ox}px;",
                                // )
                            />
                            // <circle
                            // class="rotator"
                            // style=format!("transform-origin: {ox}px {oy}px;")
                            // fill=lan.color
                            // cx=IMAGE_RADIUS
                            // cy=IMAGE_RADIUS
                            // r=IMAGE_RADIUS
                            // />
                            <circle fill="black" cx=*ox cy=*oy r=10 />
                        </g>
                    }
                })
                .collect_view()}
        </svg>
    }
}
