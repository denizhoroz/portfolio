use dioxus::prelude::*;

mod components;
// Glob-imported, not listed: the Route derive below needs Home, MyWorksPage,
// WorkPage and Navbar in scope by name, and this is also what lets one component
// module reach another through `crate::` (private imports are visible to
// descendant modules).
use components::*;

// No `use data::*` here any more -- main.rs itself touches none of it. Each
// component reaches what it needs directly, e.g. `crate::data::fetch_projects`.
mod data;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]

    #[route("/")]
    Home {},

    #[route("/works")]
    MyWorksPage {},

    #[route("/works/:slug")]
    WorkPage { slug: String }
}

// Document-level assets. Everything else is declared next to the component that
// renders it -- the social icons live in components/reach_me.rs, the GitHub icon
// on the project page in components/work.rs.
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
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "anonymous" }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Google+Sans:ital,opsz,wght@0,17..18,400..700;1,17..18,400..700&display=swap"
        }

        Router::<Route> {}
    }
}
