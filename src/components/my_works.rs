use dioxus::prelude::*;

use crate::{DataErrorBlock, ProjectBox, Route, SkeletonGrid, data::fetch_projects};

// MyWorks section
/// How many projects the home page previews. The rest are on /works.
const HOME_PROJECT_COUNT: usize = 3;

#[component]
pub fn MyWorks() -> Element {
    let mut projects = use_resource(|| async move { fetch_projects().await });
    let state = projects.value();

    // Only the grid swaps on state. The heading and the "see my other works"
    // link render in every case, so the section never collapses mid-load.
    let grid = match &*state.read() {
        None => rsx! { SkeletonGrid { count: HOME_PROJECT_COUNT } },

        Some(Err(e)) => rsx! {
            DataErrorBlock {
                title: "Couldn't load my projects.",
                detail: "{e}",
                retryable: e.is_retryable(),
                // clear() first: restart() only flips state to Pending and
                // leaves the old value in place until the refetch resolves, so
                // without this the error panel stays up and the button looks dead.
                on_retry: move |_| { projects.clear(); projects.restart(); },
            }
        },

        Some(Ok(list)) if list.is_empty() => rsx! {
            p { class: "block-desc", "Nothing here yet." }
        },

        Some(Ok(list)) => rsx! {
            div {
                class: "project-container",

                for p in list.iter().take(HOME_PROJECT_COUNT) {
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
        div {
            id: "myworks",
            // Below lg this section is taller than one screen (3 cards at
            // min-h-[28rem], 1 column under 640px and 2 under 1024px), so
            // .page-section is a minimum here, not the actual height.
            class: "page-block page-section",
            div {
                class: "flex w-full flex-col items-center justify-center gap-5",
                h2 {
                    class: "block-title",
                    "my works"
                }

                {grid}

                Link { class: "block-desc button", to: Route::MyWorksPage {},  "see my other works" }

            }
        }
    }
}
