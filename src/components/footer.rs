use dioxus::prelude::*;

// Footer section
#[component]
pub fn Footer() -> Element {
    rsx! {
        div {
            class: "mx-auto text-center text-subtle",
            "denizhoroz - 2026"
        }
    }
}
