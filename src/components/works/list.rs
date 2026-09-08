use dioxus::prelude::*;

use crate::ProjectGrid;

/// Placeholders while loading -- the expected count, not a promise.
const SKELETON_COUNT: usize = 6;

#[component]
pub fn MyWorksPage() -> Element {
    rsx! {
        document::Title { "All works — denizhoroz" }

        div {
            class: "page-block",

            h1 { class: "block-title m-2.5", "all my works" }

            ProjectGrid {
                skeleton_count: SKELETON_COUNT,
                empty: "No projects here yet.",
            }
        }
    }
}
