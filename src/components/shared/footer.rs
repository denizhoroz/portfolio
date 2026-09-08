use dioxus::prelude::*;

// Footer section
#[component]
pub fn Footer() -> Element {
    rsx! {
        // Two elements on purpose: the outer is full-bleed and exists only to
        // draw the rule across the whole page, the inner keeps the text in the
        // shared measure. --bar-height is the same variable .navbar-inner uses,
        // so the footer matches the navbar exactly and cannot drift.
        div {
            class: "border-t border-edge",
            div {
                class: "page-block min-h-[var(--bar-height)] text-subtle",
                "denizhoroz - 2026"
            }
        }
    }
}
