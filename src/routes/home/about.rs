use leptos::prelude::*;
use leptos::{component, view, IntoView};

#[component()]
pub fn about() -> impl IntoView {
    view! {
        <p class="lg:w-[60%] text-center content-center m-auto mt-8">
            Hi there! Welcome to my homepage ={")"} <br />I am a Swtizerland based developer. <br />
            I work at <a href="http://coop.ch">Coop</a>
            as a System architect and Product Owner. I pretty much work on devops for the systems present in every store in Switzerland.

            <br />

        </p>
    }
}
