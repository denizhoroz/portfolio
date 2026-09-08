use dioxus::prelude::*;

// Title section
#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            class: "page-block page-section",
            div {
                // The one h1 on the page. "Software Engineer" was a second h1;
                // it is a tagline, not a heading, so it is now a <p>.
                h1 {
                    class: "block-title",
                    "denizhoroz"
                }

                p {
                    class: "block-subtitle",
                    "Software Engineer"
                }
            }
        }
    }
}
