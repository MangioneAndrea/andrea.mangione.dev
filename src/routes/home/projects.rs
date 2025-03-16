use leptos::html::{Canvas, Img, Input};
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::svg::Image;
use leptos::{component, view, IntoView};

use web_sys::js_sys::Uint8ClampedArray;
use web_sys::wasm_bindgen::prelude::Closure;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{js_sys, Blob, File, FileReader, HtmlInputElement, ImageData, ProgressEvent, Url};

#[component()]
pub fn projects() -> impl IntoView {
    let file_input: NodeRef<Input> = NodeRef::new();
    let image_original: NodeRef<Img> = NodeRef::new();
    let canvas_target: NodeRef<Canvas> = NodeRef::new();

    let on_dither_input_change = move |target: HtmlInputElement| -> Option<()> {
        let files = target.files()?;
        let file: File = files.get(0)?;

        let blob = Blob::from(file);

        let as_url = Url::create_object_url_with_blob(&blob).unwrap();
        image_original.get()?.set_src(&as_url.as_str());

        let fr = FileReader::new().ok()?;

        let mut ctx = canvas_target
            .get()
            .expect("")
            .get_context("2d")
            .expect("")
            .expect("")
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .expect("");
        console_log("hey");

        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            console_log(format!("{event:?}").as_str());

            if let Ok(_) = event.target().expect("").dyn_into::<ProgressEvent>() {
                console_log(format!("nope").as_str());
                return;
            }

            let fr = event
                .target()
                .expect("")
                .dyn_into::<FileReader>()
                .expect("");
            console_log("hey2");

            let result = fr.result().expect("");

            console_log("hlley");
            let array_buffer: js_sys::ArrayBuffer = result.dyn_into().expect("");
            console_log("hey4");
            let uint8_array = Uint8ClampedArray::new(&array_buffer);
            let vec_u8 = uint8_array.to_vec();
            console_log(format!("{vec_u8:?}").as_str());

            let w = 548.;
            let h = 189.;
            console_log("hey??");
            let img_data =
                ImageData::new_with_js_u8_clamped_array_and_sh(&uint8_array, w as _, h as _)
                    .expect("");
            console_log("hey5");
            ctx.put_image_data(&img_data, w, h).unwrap();
            console_log("hey6");
            // console_log(format!("{vec_u8:?}").as_str());
        }) as Box<dyn FnMut(web_sys::Event)>);

        fr.set_onloadend(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
        fr.read_as_array_buffer(&blob).ok()?;

        Some(())
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
                on_dither_input_change(evt.target());
            }
        />

        <img node_ref=image_original />
        <canvas node_ref=canvas_target />
    }
}
