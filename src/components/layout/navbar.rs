use leptos::prelude::*;

use crate::components::icons::{Arrow, Circle, Copy, Github, LinkedIn, Logo};

#[component]
fn NavLogo(open: RwSignal<bool>) -> impl IntoView {
    view! {
        <a href="/" class="relative w-fit h-fit">
            <div class="relative w-full h-full flex items-center justify-center gap-3 py-10">
                <div
                    data-open=move || open.get().to_string()
                    class="font-heading relative grid h-15 w-15 place-content-center rounded-full bg-black data-[open=true]:bg-white"
                >
                    <Logo
                        attr:data-open=move || open.get().to_string()
                        class={Some("h-12 w-12 text-white data-[open=true]:text-black")} />
                </div>
                <h1
                    data-open=move || open.get().to_string()
                    class="font-heading text-lg font-medium text-black data-[open=true]:text-white"
                >
                    bnjfolio.dev
                </h1>
            </div>
        </a>
    }
}

#[component]
fn NavEmail(open: RwSignal<bool>) -> impl IntoView {
    view! {
        <div
        data-open=move || open.get().to_string()
        class="relative hidden h-full max-h-[33px] w-full max-w-[228px] rounded-full bg-black text-white px-3 py-2 data-[open=true]:bg-white data-[open=true]:text-black md:block"
        >
            <div class="flex h-full w-full items-center justify-center gap-1">
                <Copy class=Some("h-5 w-5") />
                <p class="font-heading text-xs font-medium">benjiebengarcia@gmail.com</p>
            </div>
        </div>

    }
}

#[component]
fn NavClose(open: RwSignal<bool>) -> impl IntoView {
    let toggle = move |_| open.set(!open.get());

    view! {
        <button
        on:click={toggle}
        data-open=move || open.get().to_string()
        class="grid h-16 w-16 cursor-pointer place-content-center rounded-full bg-black text-white active:scale-98 active:transition-all data-[open=true]:bg-white lg:h-24 lg:w-24 data-[open=true]:[&>svg]:text-black"
        >
            <Arrow
            attr:data-rotate=move || {open.get().to_string()}
            class=Some("h-3 w-3 data-[rotate=false]:rotate-90 lg:h-6 lg:w-6") />
        </button>
    }
}

#[component]
fn NavLink(path: Option<&'static str>, label: &'static str) -> impl IntoView {
    view! {
        <li>
            <a
                href={path.unwrap_or_default()}
                class="font-heading group flex min-w-fit items-center justify-center gap-12 text-4xl font-normal text-white uppercase md:text-6xl lg:text-9xl"
            >
                <Arrow
                class=Some("group-hover:text-accent hidden h-10 w-10 rotate-45 group-hover:block group-hover:scale-98 group-hover:transition-all")
            />
                <p class="group-hover:text-accent group-hover:scale-98 group-hover:transition-all">
                    {label}
                </p>
            </a>
        </li>
    }
}

#[component]
fn Socials() -> impl IntoView {
    view! {
        <li class="flex flex-col items-center justify-center gap-6 md:flex-row">
            <a
                href="https://github.com/mystique09/"
                target="_blank"
                rel="noopener noreferrer"
                class="flex h-full min-h-fit w-full min-w-fit items-center justify-center gap-2 rounded-full bg-white px-14 py-3 hover:scale-98 hover:transition-all"
            >
                <Github class=Some("text-primary h-4 w-4") />
                <p class="text-primary font-heading text-xs font-medium">Follow me on Github</p>
            </a>
            <a
                href="https://www.linkedin.com/in/benjie-ben-garcia-916261202/"
                target="_blank"
                rel="noopener noreferrer"
                class="flex h-full min-h-fit w-full min-w-fit items-center justify-center gap-2 rounded-full bg-white px-14 py-3 hover:scale-98 hover:transition-all"
            >
                <LinkedIn class=Some("text-primary h-4 w-4") />
                <p class="text-primary font-heading text-xs font-medium">"Let's connect on LinkedIn"</p>
            </a>
        </li>
    }
}

#[component]
pub fn Navbar() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <header
        // in:slide
        data-open=move || open.get().to_string()
        class="bg-white data-[open=true]:bg-primary sticky data-[open=false]:-top-161 left-0 z-50 max-h-screen data-[open=true]:h-screen w-full overflow-hidden px-4 data-[open=true]:top-0 lg:px-0"
        >
        <div
            // data-open={open}
            class="relative mx-auto flex h-full w-full max-w-7xl flex-col transition-all duration-500 ease-in-out"
        >
            // {@render NavLinks()}
            <div class="flex items-center justify-between">
                <NavLogo open={open} />
                <NavEmail open={open} />
                <NavClose open={open} />
            </div>
            <div
                data-open=move || open.get().to_string()
                class="relative h-full w-full transition-all duration-500 ease-in-out hidden data-[open=true]:block"
            >
                <nav
                //  in:fade={{ delay: 500 }}
                class="relative h-full w-full">
                    <ul
                        class="flex h-full w-full flex-col items-center justify-start py-20 gap-5 md:items-end"
                    >
                        <NavLink label="Home" path=None />
                        <NavLink label="About" path=Some("/#about") />
                        <NavLink label="Projects" path=Some("/#projects") />
                        <Socials />
                    </ul>
                </nav>
                <div
                // in:fade={{ delay: 500 }}
                >
                    <Circle
                        attr:data-open={move || open.get().to_string()}
                        class=Some("absolute -bottom-40 -left-82 z-0 hidden h-[26.25rem] w-[26.25rem] text-white data-[open=true]:block")
                    />
                </div>
            </div>
        </div>
    </header>
    }
}
