use dioxus::prelude::*;

use crate::{ProjectGrid, Route, components::Footer};

/// Placeholders while loading -- the expected count, not a promise.
const SKELETON_COUNT: usize = 6;

#[component]
pub fn WorksPage() -> Element {
    rsx! {
        div {
            class: "page-block",

            Link { class: "block-desc button", to: Route::Home {}, "go back" }

            h1 { class: "block-title m-2.5", "all my works" }

            ProjectGrid {
                skeleton_count: SKELETON_COUNT,
                empty: "No projects here yet.",
            }
        }

        div {
            class: "pt-[clamp(4rem,12vw,10rem)]",
            Footer {}
        }        
    }
}
