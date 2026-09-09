use dioxus::prelude::*;

use crate::{ArticleList, Breadcrumb, Crumb, Route};

/// Placeholders while loading -- the expected count, not a promise.
const SKELETON_COUNT: usize = 4;

#[component]
pub fn ArticlesPage() -> Element {
    rsx! {
        div {
            class: "page-block",

            Breadcrumb {
                trail: vec![
                    Crumb::link("home", Route::Home {}),
                    Crumb::current("articles"),
                ],
            }

            h1 { class: "block-title m-2.5", "my articles" }

            ArticleList {
                skeleton_count: SKELETON_COUNT,
                empty: "No articles here yet.",
            }
        }

    }
}
