use dioxus::prelude::*;

use crate::{ArticleList, Route};

/// Placeholders while loading -- the expected count, not a promise.
const SKELETON_COUNT: usize = 4;

#[component]
pub fn ArticlesPage() -> Element {
    rsx! {
        div {
            class: "page-block",

            Link { class: "block-desc button mb-[20px]", to: Route::Home {}, "go back" }

            h1 { class: "block-title m-2.5", "my articles" }

            ArticleList {
                skeleton_count: SKELETON_COUNT,
                empty: "No articles here yet.",
            }
        }

    }
}
