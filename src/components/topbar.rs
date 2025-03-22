use leptos::prelude::*;

use crate::routes::Routes;

#[component]
pub fn image_link(
    image: &'static str,
    link: &'static str,
    text: &'static str,
    #[prop(default = false)] with_text: bool,
) -> impl IntoView {
    view! {
        <a href=link target="_blank">
            <img class=if with_text { "w-4" } else { "w-8" } src=image alt=text />
            {if with_text { text } else { "" }}
        </a>
    }
}

#[component]
pub fn topbar() -> impl IntoView {
    view! {
        <div class="navbar bg-base-100 shadow-sm">
            <div class="navbar-start">
                <div class="dropdown">
                    <div tabindex="0" role="button" class="btn btn-ghost lg:hidden">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-5 w-5"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M4 6h16M4 12h8m-8 6h16"
                            />
                        </svg>
                    </div>
                    <ul
                        tabindex="0"
                        class="menu menu-sm dropdown-content bg-base-100 rounded-box z-1 mt-3 w-52 p-2 shadow"
                    >
                        <li>
                            <ImageLink
                                link=Routes::Home.path()
                                text="Home"
                                image="/assets/icons/home.svg"
                                with_text=true
                            />
                        </li>
                        <li>
                            <ImageLink
                                link=Routes::Blog.path()
                                text="Blog"
                                image="/assets/icons/blog.svg"
                                with_text=true
                            />
                        </li>
                        <li>
                            <ImageLink
                                link="https://github.com/MangioneAndrea"
                                text="Github"
                                image="/assets/Logos/Github.webp"
                                with_text=true
                            />
                        </li>
                        <li>
                            <ImageLink
                                link="https://www.linkedin.com/in/andrea-mangione-592902156/"
                                text="LinkedIn"
                                image="/assets/Logos/Linkedin.webp"
                                with_text=true
                            />
                        </li>
                        <li>
                            <ImageLink
                                link="mailto:andrea@mangione.dev"
                                text="Email"
                                image="/assets/Logos/Email.webp"
                                with_text=true
                            />
                        </li>
                    </ul>
                </div>
                <label class="hidden lg:flex">andrea.mangione.dev</label>
            </div>

            <div class="navbar-center">
                <label class="lg:hidden">andrea.mangione.dev</label>
                <div role="tablist" class="tabs tabs-border hidden lg:flex">
                    <a
                        role="tab"
                        class="tab "
                        class:tab-active=|| Routes::get_active() == Routes::Home
                        href=Routes::Home.path()
                    >
                        Home
                    </a>
                    <a
                        role="tab"
                        class="tab"
                        class:tab-active=|| Routes::get_active() == Routes::Blog
                        href=Routes::Blog.path()
                    >
                        Blog
                    </a>
                </div>
            </div>
            <div class="navbar-end">
                <ul class="menu menu-horizontal px-1 hidden lg:flex">
                    <button class="btn btn-link">
                        <ImageLink
                            link="https://github.com/MangioneAndrea"
                            text="Github"
                            image="/assets/Logos/Github.webp"
                        />
                    </button>
                    <button class="btn btn-link">
                        <ImageLink
                            link="https://www.linkedin.com/in/andrea-mangione-592902156/"
                            text="LinkedIn"
                            image="/assets/Logos/Linkedin.webp"
                        />
                    </button>
                    <button class="btn btn-link">
                        <ImageLink
                            link="mailto:andrea@mangione.dev"
                            text="Email"
                            image="/assets/Logos/Email.webp"
                        />
                    </button>
                </ul>
            </div>
        </div>
    }
}
