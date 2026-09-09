use dioxus::prelude::*;

use crate::{ArticleList, Route};

/// How many articles the home page previews. The rest are on /articles.
const HOME_ARTICLE_COUNT: usize = 3;

#[component]
pub fn Articles() -> Element {
    rsx! {
        div {
            id: "myarticles",
            // Three stacked rows are taller than one screen on a phone, where
            // each row's image and text sit one above the other, so
            // .page-section is a minimum here rather than the actual height.
            class: "page-block page-section",
            div {
                class: "flex w-full flex-col items-center justify-center gap-5",
                h2 { class: "block-title", "latest articles" }

                ArticleList {
                    skeleton_count: HOME_ARTICLE_COUNT,
                    limit: HOME_ARTICLE_COUNT,
                    empty: "Nothing here yet.",
                }

                Link { class: "block-desc button", to: Route::ArticlesPage {}, "see all articles" }
            }
        }
    }
}
