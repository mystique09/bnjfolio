use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::components::{features::hero::Hero, layout::Navbar};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="icon" href="/favicon.png" />
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/bnjfolio-dev.css"/>
        <Stylesheet href="/fonts/space-grotesk/space-grotesk-latin-300-normal.woff" />
        <Stylesheet href="/fonts/space-grotesk/space-grotesk-latin-300-normal.woff2" />
        <Stylesheet href="/fonts/space-grotesk/space-grotesk-latin-400-normal.woff" />
        <Stylesheet href="/fonts/space-grotesk/space-grotesk-latin-400-normal.woff2" />
        <Stylesheet href="/fonts/space-grotesk/space-grotesk-latin-500-normal.woff" />
        <Stylesheet href="/fonts/space-grotesk/space-grotesk-latin-500-normal.woff2" />
        <Stylesheet href="/fonts/space-grotesk/space-grotesk-latin-600-normal.woff" />
        <Stylesheet href="/fonts/space-grotesk/space-grotesk-latin-600-normal.woff2" />
        <Stylesheet href="/fonts/space-grotesk/space-grotesk-latin-700-normal.woff" />
        <Stylesheet href="/fonts/space-grotesk/space-grotesk-latin-700-normal.woff2" />

        <Stylesheet href="/fonts/roboto/roboto-latin-400-normal.woff2" />
        <Stylesheet href="/fonts/roboto/roboto-latin-500-normal.woff2" />
        <Stylesheet href="/fonts/roboto/roboto-latin-700-normal.woff2" />

        // sets the document title
        <Title text="bnjbn.g"/>
        <meta
            name="description"
            content="I specialize in Front-End Development and crafting Responsive Web Designs that deliver seamless user experiences across all devices."
        />

        // content for this welcome page
        <Router>
            <Navbar />
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Hero />
    }
}
