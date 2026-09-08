use dioxus::prelude::*;
use crate::{DataErrorBlock, ProjectBox, Route, SkeletonGrid, data::fetch_projects};

/// How many placeholders to show while loading. Cosmetic only -- it is the
/// expected count, not a promise about what will arrive.
const SKELETON_COUNT: usize = 6;

#[component]
pub fn MyWorksPage() -> Element {
    let mut projects = use_resource(|| async move { fetch_projects().await });
    let state = projects.value();

    let body = match &*state.read() {
        None => rsx! { SkeletonGrid { count: SKELETON_COUNT } },

        Some(Err(e)) => rsx! {
            DataErrorBlock {
                title: "Couldn't load my projects.",
                detail: "{e}",
                retryable: e.is_retryable(),
                // clear() before restart(): restart() alone leaves the stale
                // value, so the error panel would sit there looking unresponsive.
                on_retry: move |_| { projects.clear(); projects.restart(); },
            }
        },

        Some(Ok(list)) if list.is_empty() => rsx! {
            p { class: "block-desc", "No projects here yet." }
        },

        Some(Ok(list)) => rsx! {
            div {
                class: "project-container",

                for p in list.iter() {
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

    rsx! {
        document::Title { "All works — denizhoroz" }

        // No navbar offset here any more. The old lg:pt-60 existed only to clear
        // the fixed left rail (y=20..338), which this page's grid ran straight
        // through at lg. The rail is gone -- the navbar is a top bar at every
        // width now -- so body's padding-top: var(--nav-clearance) clears it and
        // keeping the 15rem would just be dead space.
        div {
            class: "page-block",

            h1 {
                class: "block-title m-2.5",
                "all my works"
            }

            {body}
        }
    }
}
