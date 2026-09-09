use dioxus::prelude::*;

use crate::{Breadcrumb, Crumb, ProjectGrid, Route};

/// Placeholders while loading -- the expected count, not a promise.
const SKELETON_COUNT: usize = 6;

#[component]
pub fn WorksPage() -> Element {
    rsx! {
        div {
            class: "page-block",

            Breadcrumb {
                trail: vec![
                    Crumb::link("home", Route::Home {}),
                    Crumb::current("works"),
                ],
            }

            h1 { class: "block-title m-2.5", "my works" }

            ProjectGrid {
                skeleton_count: SKELETON_COUNT,
                empty: "No projects here yet.",
            }
        }
        
    }
}
