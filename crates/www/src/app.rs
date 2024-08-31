use crate::{components::*, routes::index::IndexPage};
use leptos::prelude::*;
use leptos_meta::{
    provide_meta_context, MetaTags, Stylesheet, Title,
};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" class="h-full bg-ctp-base">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options=options islands=true/>
                <MetaTags/>
                <link rel="preconnect" href="https://fonts.googleapis.com"/>
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
                <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@100..900&display=swap" rel="stylesheet"/>
            </head>
            <body class="h-full">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets,
    // titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/www.css"/>

        <Title text="Chris Biscardi"/>

        <ProgressBar/>

        <Router>
            <main class="relative textured-bg">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=IndexPage/>
                </Routes>
            </main>
        </Router>
        <Footer/>
    }
}
