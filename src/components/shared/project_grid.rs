use dioxus::prelude::*;

use crate::{ProjectBox, Route, SkeletonGrid, data::fetch_projects};

/// The card grid, shared by Home's preview and the full /works page.
///
/// Those two differed only in how many cards they showed and what they say when
/// there are none, so everything else -- the resource, the loading state, the
/// links, the keys -- lives here once.
#[component]
pub fn ProjectGrid(
    /// Placeholders to show while loading; the expected count, not a promise.
    skeleton_count: usize,
    /// Cap on how many cards to render. `None` renders all of them.
    limit: Option<usize>,
    /// Shown when the source returns nothing.
    empty: String,
) -> Element {
    let projects = use_resource(|| async move { fetch_projects().await });
    let state = projects.value();

    // Bound to a local rather than returned directly: the match borrows the
    // read guard, and as a tail expression that borrow outlives `state`.
    let grid = match &*state.read() {
        None => rsx! { SkeletonGrid { count: skeleton_count } },

        Some(list) if list.is_empty() => rsx! {
            p { class: "block-desc", "{empty}" }
        },

        Some(list) => rsx! {
            div {
                class: "project-container",

                for p in list.iter().take(limit.unwrap_or(usize::MAX)) {
                    Link {
                        to: Route::WorkPage { slug: p.slug.clone() },
                        key: "{p.slug}",
                        class: "group",
                        ProjectBox { project: p.clone() }
                    }
                }
            }
        },
    };

    grid
}
