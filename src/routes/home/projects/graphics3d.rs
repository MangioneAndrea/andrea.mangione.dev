use leptos::prelude::*;
use leptos::{component, view, IntoView};

#[component()]
pub fn graphics() -> impl IntoView {
    view! {
        <span class="flex justify-between">
            <h3 class="flex">
                <a
                    class="link link-primary"
                    href="https://github.com/MangioneAndrea/graphics-3d"
                    target="_blank"
                >
                    Graphics 3d
                </a>
            </h3>
            <span class="flex items-end">
                <img src="assets/Logos/Rust.png" class="w-5 h-5 mt-auto ml-auto" />
            </span>
        </span>
        <p>
            Simple setup for pixel drawing playground <br />
            From time to time i like to try out drawing something directly on a screen buffer. This project is where i put it all...
            <br />... as an example here is a simple path tracer, from the
            <a class="link" href="https://raytracing.github.io/books/RayTracingInOneWeekend.html">
                {"\"Ray Tracing In One Weekend\""}
                series
            </a> <br />The whole thing runs on cpu, a cool feature would be to add cuda or vulkan
            <br />The project is setup on linux, but it should work (not tested) on windows as well.
        </p>
        <div class="flex flex-col lg:flex-row justify-around">
            <img
                class="lg:max-w-[60%] lg:w-[60%] border-2 border-solid m-4"
                src="assets/Images/path_tracer.png"
            />
        </div>
    }
}
