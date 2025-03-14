use crate::components::pin_wheel::{Image, PinWheel};
use leptos::prelude::*;
use leptos::{component, view, IntoView};

#[component()]
pub fn stack() -> impl IntoView {
    view! {
        <h2 id="stack">
            <a href="#stack">My stack</a>
        </h2>
        <div class="flex flex-col lg:flex-row justify-evenly">
            <p class=" lg:w-[30%] text-center content-center">
                There are tons of web framework, I tryed and used a couple even professionally like Angular, React and NextJS.
                <br />
                The only ones I would use if starting from scratch are the simpler ones: the ones which will not cause 20000 rerender because of a wrong useEffect ...
                <br />...This website is powered by leptos btw. :D
            </p>
            <div class="flex justify-around lg:w-[30%] ">
                <PinWheel images=vec![
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
            </div>
        </div>
        <div class="flex flex-col lg:flex-row-reverse justify-evenly">
            <p class="lg:w-[30%] text-center content-center">
                As for non-web stuff, my primary choise is pretty much always Rust. From times to times I use JS, mostly where something hast to be done quick and dirty. Similar story for go, if it is a small tool I use Go, as it is the easiest language I know
            </p>
            <div class="flex justify-around lg:w-[30%]">
                <PinWheel images=vec![
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
            </div>
        </div>

        <div class="flex flex-col lg:flex-row justify-evenly">
            <p class=" lg:w-[30%] text-center content-center">
                But computer sience is way way more! <br />
                I am very passionate about Neovim, here some of the technologies that i use over and over again...
            </p>
            <div class="flex justify-around lg:w-[30%]">
                <PinWheel images=vec![
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
                    Image {
                        src: "assets/Logos/Esp32.png",
                        key: "esp32",
                    },
                ] />
            </div>
        </div>
    }
}
