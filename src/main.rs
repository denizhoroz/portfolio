use dioxus::prelude::*;

mod components;
// Glob-imported: the Route derive needs these in scope by name, and it is also
// what lets one component module reach another through `crate::`.
use components::*;

mod data;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]

    #[route("/")]
    Home {},

    #[route("/works")]
    WorksPage {},

    #[route("/works/:slug")]
    WorkPage { slug: String }
}

// Document-level assets; everything else is declared next to the component that
// renders it.
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}


#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        // The webfont is loaded from index.html, not here: document::Link is
        // inserted once WASM boots, which is late enough to paint the whole
        // first screen in the fallback face and then reflow it.

        Router::<Route> {}
    }
}
