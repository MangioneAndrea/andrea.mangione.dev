use crate::components::stack_visualizer::{Image, StackVisualizer};
use leptos::prelude::ElementChild;
use leptos::{component, view, IntoView};

#[component()]
pub fn stack() -> impl IntoView {
    view! {
        <h2>My stack</h2>
        <StackVisualizer images=vec![
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
        ] />
        <StackVisualizer images=vec![
            Image {
                src: "assets/Logos/Go.svg",
                key: "Golang",
            },
            Image {
                src: "assets/Logos/Kotlin.svg",
                key: "Kotlin",
            },
            Image {
                src: "assets/Logos/NodeJS.svg",
                key: "Node",
            },
            Image {
                src: "assets/Logos/Ferris.svg",
                key: "Rust",
            },
        ] />

        <StackVisualizer images=vec![
            Image {
                src: "assets/Logos/GraphQL.svg",
                key: "graphql",
            },
            Image {
                src: "assets/Logos/MongoDB.svg",
                key: "mongo",
            },
            Image {
                src: "assets/Logos/Grpc.svg",
                key: "grpc",
            },
            Image {
                src: "assets/Logos/Docker.svg",
                key: "docker",
            },
            Image {
                src: "assets/Logos/Neovim2.png",
                key: "nvim",
            },
        ] />
    }
}
