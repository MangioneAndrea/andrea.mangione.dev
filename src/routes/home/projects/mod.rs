use std::time::Duration;

use leptos::html::{Canvas, Img, Input};
use leptos::leptos_dom::logging::console_error;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::{component, view, IntoView};

use web_sys::wasm_bindgen::{Clamped, JsCast};
use web_sys::{js_sys, Blob, File, FileReader, HtmlInputElement, ImageData, ProgressEvent, Url};

#[component()]
pub fn projects() -> impl IntoView {
    let file_input: NodeRef<Input> = NodeRef::new();
    let image_original: NodeRef<Img> = NodeRef::new();
    let canvas_target: NodeRef<Canvas> = NodeRef::new();

    let (dimensions, set_dimensions) = signal((400, 400));

    let on_img_change = move || -> Result<(), &'static str> {
        let img = image_original.get().ok_or("Couldn't get original image from node")?;

        let w = img.natural_width();
        let h = img.natural_height();
        set_dimensions.set((w,h));


        let ctx = canvas_target
            .get()
            .ok_or("Could not get canvas from node")?
            .get_context("2d")
            .map_err(|_|"Could not get context")?
            .ok_or("Context is empty")?
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .expect("Unreachable! Get context('2d') must return 2d context");

        ctx.draw_image_with_html_image_element(&image_original.get().unwrap(), 0., 0.).map_err(|_|"Could not draw original image")?;

        let img_data = ctx.get_image_data(0., 0., w as _, h as _).map_err(|_|"Failed to get image data")?;

        let d_image = ditherrific::Image::from_rgb(w, h, img_data.data().to_vec());

        let result = ditherrific::transform(d_image, ditherrific::algorithms::Options::SierraLite);

        let pixels = result.to_rgb();

        let image_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&pixels.0[..]), w, h).map_err(|_|"Failed to create image data")?;

        ctx.put_image_data(&image_data, 0., 0.).map_err(|_|"Failed to print result")?;
        let ctx = canvas_target
            .get()
            .ok_or("Could not get canvas from node")?
            .get_context("2d")
            .map_err(|_|"Could not get context")?
            .ok_or("Context is empty")?
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .expect("Unreachable! Get context('2d') must return 2d context");

        ctx.draw_image_with_html_image_element(&image_original.get().unwrap(), 0., 0.).map_err(|_|"Could not draw original image")?;

        let img_data = ctx.get_image_data(0., 0., w as _, h as _).map_err(|_|"Failed to get image data")?;

        let d_image = ditherrific::Image::from_rgb(w, h, img_data.data().to_vec());

        let result = ditherrific::transform(d_image, ditherrific::algorithms::Options::SierraLite);

        let pixels = result.to_rgb();

        spawn_local(async move {
            let image_data = Box::new(ImageData::new_with_u8_clamped_array_and_sh(Clamped(&pixels.0[..]), w, h).map_err(|_|"Failed to create image data").unwrap());


            ctx.put_image_data(Box::leak(image_data), 0., 0.).map_err(|_|"Failed to print result").unwrap();

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
        <h2 id="projects">
            <a href="#projects">Projects</a>
        </h2>

        <h3>Ditherrific</h3>
        <input
            type="file"
            node_ref=file_input
            on:change:target=move |evt| {
                on_dither_input_change(evt.target()).unwrap_or_else(console_error);
            }
        />

        <div class="flex flex-col lg:flex-row justify-around">
            <img class="lg:max-w-[35%] border-2 border-solid m-4" node_ref=image_original src="assets/Images/parrots.png" on:load=move |_|{on_img_change().unwrap_or_else(console_error);} />
            <canvas
                width=move ||dimensions.get().0
                height=move ||dimensions.get().1
                class="lg:max-w-[35%] border-2 border-solid m-4" node_ref=canvas_target 
            />
        </div>
    }
}
