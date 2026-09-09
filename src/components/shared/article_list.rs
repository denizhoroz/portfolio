use dioxus::prelude::*;

use crate::{ArticleRow, Route, SkeletonRowList, data::fetch_articles};

/// The stacked article list, shared by Home's preview and the full /articles
/// page.
///
/// The projects' counterpart is `ProjectGrid`, and this mirrors it exactly:
/// those two callers differ only in how many rows they show and what they say
/// when there are none, so everything else -- the resource, the loading state,
/// the links, the keys -- lives here once.
#[component]
pub fn ArticleList(skeleton_count: usize, limit: Option<usize>, empty: String) -> Element {
    let articles = use_resource(|| async move { fetch_articles().await });
    let state = articles.value();

    // Bound to a local rather than returned directly: the match borrows the
    // read guard, and as a tail expression that borrow outlives `state`.
    let list = match &*state.read() {
        None => rsx! { SkeletonRowList { count: skeleton_count } },

        Some(list) if list.is_empty() => rsx! {
            p { class: "block-desc", "{empty}" }
        },

        Some(list) => rsx! {
            div {
                class: "article-container",

                for a in list.iter().take(limit.unwrap_or(usize::MAX)) {
                    Link {
                        to: Route::ArticlePage { slug: a.slug.clone() },
                        key: "{a.slug}",
                        class: "group",
                        ArticleRow { article: a.clone() }
                    }
                }
            }
        },
    };

    list
}
