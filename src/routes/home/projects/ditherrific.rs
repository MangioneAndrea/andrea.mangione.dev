use std::str::FromStr;
use ditherrific::algorithms::Options;
use leptos::html::{Canvas, Img, Input};
use leptos::leptos_dom::logging::console_error;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::{component, view, IntoView};

use strum::IntoEnumIterator;
use web_sys::wasm_bindgen::{Clamped, JsCast};
use web_sys::{Blob, File, HtmlCanvasElement, HtmlImageElement, HtmlInputElement, ImageData, Url};

#[inline(never)]
async fn draw_on_canvas(w: u32, h: u32, canvas_target: HtmlCanvasElement, image_original: HtmlImageElement, method: Options)  -> Result<(), &'static str> {
        let ctx = canvas_target
            .get_context("2d")
            .map_err(|_|"Could not get context")?
            .ok_or("Context is empty")?
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .expect("Unreachable! Get context('2d') must return 2d context");


        ctx.draw_image_with_html_image_element(&image_original, 0., 0.).map_err(|_|"Could not draw original image")?;

        let img_data = ctx.get_image_data(0., 0., w as _, h as _).map_err(|_|"Failed to get image data")?;

        let d_image = ditherrific::Image::from_rgb(w, h, img_data.data().to_vec());

        let result = ditherrific::transform(d_image, method);

        let pixels = result.to_rgb();

        let image_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&pixels.0[..]), w, h).map_err(|_|"Failed to create image data")?;

        ctx.put_image_data(&image_data, 0., 0.).map_err(|_|"Failed to print result")?;
    Ok(())
}

#[component()]
pub fn ditherrific() -> impl IntoView {
    let file_input: NodeRef<Input> = NodeRef::new();
    let image_original: NodeRef<Img> = NodeRef::new();
    let canvas_target: NodeRef<Canvas> = NodeRef::new();

    let (dimensions, set_dimensions) = signal((400, 400));
    let (selected_option, set_selected_option) = signal(Options::FloydSteinberg);


    let redraw = move || -> Result<(), &'static str> {
        let image_original = image_original.get().ok_or("Couldn't get original image from node")?;
        let canvas_target = canvas_target.get()
            .ok_or("Could not get canvas from node")?;

        let w = image_original.natural_width();
        let h = image_original.natural_height();
        set_dimensions.set((w,h));
        
        let method = selected_option.get();


            spawn_local(async move {
                draw_on_canvas(w, h, canvas_target, image_original, method).await.unwrap_or_else(console_error);
            });

        Ok(())

    };

    let on_dither_input_change = move |target: HtmlInputElement| -> Result<(), &'static str> {
        let files = target.files().ok_or("Failed to get files")?;
        let file: File = files.get(0).ok_or("No valid file selected")?;

        let blob = Blob::from(file);

        let as_url = Url::create_object_url_with_blob(&blob).map_err(|_|"Failed to create url from blob")?;
        image_original.get().ok_or("Couldn't get original image from node")?.set_src(&as_url.as_str());


        Ok(())
    };

    view! {
        <span class="flex justify-between">
            <h3 class="flex">
                <a
                    class="link link-primary"
                    href="https://github.com/MangioneAndrea/ditherrific"
                    target="_blank"
                >
                    Ditherrific
                </a>
            </h3>
            <span class="flex items-end">
                <img src="assets/Logos/Rust.png" class="w-5 h-5 mt-auto ml-auto" />
                <span class="mx-3">+</span>
                <img src="assets/Logos/Wasm.png" class="w-5 h-5 mt-auto" />
            </span>
        </span>
        <p>
            Transform any image to black and white using dithering! This project is written in Rust and fully supports WebAssembly (WASM). In fact, you can try it right here!

            <br /> <b>Disclaimer</b>: WASM can{"'"}t be optimized yet, as there{"'"}
            s no multithreading or SIMD intrinsics. Rendering large images will still work but may be slower in the browser.

            <br /> <b>Getting started</b>:
            Try selecting a method (e.g., none), then change the transformation mode to clamp instead of dither. You can also upload your own images.

            <br /> <b>And don{"'"}t worry {"—"}everything runs completely offline!</b>
        </p>
        <div class="flex flex-col lg:flex-row justify-around m-4">
            <input
                type="file"
                node_ref=file_input
                on:change:target=move |evt| {
                    on_dither_input_change(evt.target()).unwrap_or_else(console_error);
                }
            />
            <select
                class="select"
                on:change:target=move |evt| {
                    let value = evt.target().value();
                    set_selected_option
                        .set(
                            Options::from_str(value.as_str())
                                .expect("The same string is built with strum"),
                        );
                    redraw().unwrap_or_else(console_error);
                }
            >
                {Options::iter()
                    .map(|option| {
                        view! {
                            <option
                                value=option.to_string()
                                selected=move || selected_option.get() == option
                            >
                                {option.to_string()}
                            </option>
                        }
                    })
                    .collect_view()}
            </select>
        </div>

        <div class="flex flex-col lg:flex-row justify-around">
            <img
                class="lg:max-w-[35%] lg:w-[35%] border-2 border-solid m-4"
                node_ref=image_original
                src="assets/Images/parrots.png"
                on:load=move |_| {
                    redraw().unwrap_or_else(console_error);
                }
            />
            <canvas
                width=move || dimensions.get().0
                height=move || dimensions.get().1
                class="lg:max-w-[35%] border-2 border-solid m-4"
                node_ref=canvas_target
            />
        </div>
    }
}
