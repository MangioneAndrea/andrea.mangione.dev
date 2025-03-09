use leptos::{component, view, IntoView};
use leptos::prelude::ElementChild;

#[component()]
pub fn about() -> impl IntoView {
    view! { 
        <h2>About me</h2>
        <p>
        Hi there! Welcome to my homepage ={")"}

        <br/>
        I am a Swtizerland based developer.

        <br/>
        I work at <a href="http://coop.ch">Coop</a> as a System architect and Product Owner. I pretty much work on devops for the systems present in every store in Switzerland.

        <br/>



        </p>
    }
}
