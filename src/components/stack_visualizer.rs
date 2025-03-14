use leptos::prelude::ClassAttribute;
use leptos::prelude::CustomAttribute;
use leptos::prelude::ElementChild;
use leptos::{
    prelude::{CollectView, StyleAttribute},
    *,
};
use std::f64::{self, consts::PI};

#[derive(Default)]
struct Point {
    x: f64,
    y: f64,
}

struct Location {
    mask: Point,   // The center of the mask
    pivot: Point,  // Anchor for the animation
    center: Point, // The center of the image
    image: Image,
}

#[derive(Debug)]
struct Image {
    src: &'static str,
    key: &'static str,
}

const IMAGE_RADIUS: f64 = 100.;
#[component]
pub fn stack_visualizer() -> impl IntoView {
    let languages = vec![
        Image {
            src: "assets/Logos/Astro.svg",
            key: "astro",
        },
        Image {
            src: "assets/Logos/Solid.png",
            key: "solid",
        },
        Image {
            src: "assets/Logos/Svelte.svg",
            key: "svelte",
        },
    ];

    let sectors = languages.len();

    let centers: Vec<_> = languages
        .into_iter()
        .enumerate()
        .map(|(i, image)| {
            let angle = 2.0 * PI * (i as f64) / (sectors as f64);

            let center = Point {
                x: IMAGE_RADIUS * 0.6 * angle.cos() + IMAGE_RADIUS,
                y: IMAGE_RADIUS * 0.6 * angle.sin() + IMAGE_RADIUS,
            };

            let mask = Point {
                x: IMAGE_RADIUS * angle.cos() + IMAGE_RADIUS,
                y: IMAGE_RADIUS * angle.sin() + IMAGE_RADIUS,
            };

            let pivot = Point {
                x: 2. * IMAGE_RADIUS - mask.x,
                y: 2. * IMAGE_RADIUS - mask.y,
            };

            Location {
                center,
                mask,
                pivot,
                image,
            }
        })
        .collect();

    let (class, style) = stylers::style_str! {
        @keyframes rotating {
            0% {
                -ms-transform: rotate(-10deg);
                -moz-transform: rotate(-10deg);
                -webkit-transform: rotate(-10deg);
                -o-transform: rotate(-10deg);
                transform: rotate(-10deg);
            }

            50% {
                -ms-transform: rotate(10deg);
                -moz-transform: rotate(10deg);
                -webkit-transform: rotate(10deg);
                -o-transform: rotate(10deg);
                transform: rotate(10deg);
            }

            100% {
                -ms-transform: rotate(-10deg);
                -moz-transform: rotate(-10deg);
                -webkit-transform: rotate(-10deg);
                -o-transform: rotate(-10deg);
                transform: rotate(-10deg);
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
                .map(|(src, next)| {
                    view! {
                        <mask id=src.image.key>
                            <circle
                                class="rotator"
                                style=format!(
                                    "transform-origin: {}px {}px;",
                                    src.pivot.x,
                                    src.pivot.y,
                                )
                                fill="white"
                                cx=IMAGE_RADIUS
                                cy=IMAGE_RADIUS
                                r=IMAGE_RADIUS
                                stroke="black"
                                stroke-width="10px"
                            />
                            <circle
                                class="rotator"
                                style=format!(
                                    "transform-origin: {}px {}px;",
                                    src.pivot.x,
                                    src.pivot.y,
                                )
                                fill="black"
                                cx=IMAGE_RADIUS - src.mask.x + next.mask.x
                                cy=IMAGE_RADIUS - src.mask.y + next.mask.y
                                r=IMAGE_RADIUS
                            />
                        </mask>
                        <g
                            mask=format!("url(#{})", src.image.key)
                            transform=format!("translate({},{})", src.mask.x, src.mask.y)
                        >

                            <image
                                href=src.image.src
                                transform=format!(
                                    "translate({},{})",
                                    -src.mask.x + src.center.x,
                                    -src.mask.y + src.center.y,
                                )
                                height=IMAGE_RADIUS * 2.
                                width=IMAGE_RADIUS * 2.
                            />
                        </g>
                    }
                })
                .collect_view()}
        </svg>
    }
}
