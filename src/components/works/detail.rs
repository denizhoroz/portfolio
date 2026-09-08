use dioxus::prelude::*;
use crate::{DataErrorBlock, Route, SkeletonDetail, data::fetch_project};

const GITHUB_ICON: Asset = asset!("/assets/icons/github.svg");

#[component]
pub fn WorkPage(slug: String) -> Element {
    // `use_reactive!` is load-bearing, not decoration. `use_resource` captures
    // its closure once, and navigating /works/a -> /works/b reuses this
    // component and only swaps the prop -- without this the future would keep
    // fetching the slug it was first built with.
    let mut project = use_resource(use_reactive!(|slug| async move {
        fetch_project(&slug).await
    }));
    let state = project.value();

    // Four states, not two. `Ok(None)` is a real 404; `None` is "still asking".
    // Collapsing them would flash "Project not found" on every page load.
    let body = match &*state.read() {
        None => rsx! { SkeletonDetail {} },

        Some(Err(e)) => rsx! {
            div {
                class: "page-block gap-6",

                DataErrorBlock {
                    title: "Couldn't load this project.",
                    detail: "{e}",
                    retryable: e.is_retryable(),
                    // clear() before restart(); see MyWorks.
                    on_retry: move |_| { project.clear(); project.restart(); },
                }

                Link {
                    class: "block-desc button",
                    to: Route::MyWorksPage {},
                    "Back to all works"
                }
            }
        },

        // Unknown slug. Was a bare unstyled string with no way back.
        Some(Ok(None)) => rsx! {
            document::Title { "Project not found — denizhoroz" }

            div {
                class: "page-block gap-6",

                h1 {
                    class: "block-subtitle",
                    "Project not found"
                }

                p {
                    class: "block-desc zen-content",
                    "There is no project at this address. It may have been renamed, or the link may be out of date."
                }

                Link {
                    class: "block-desc button",
                    to: Route::MyWorksPage {},
                    "Back to all works"
                }
            }
        },

        Some(Ok(Some(p))) => {
            let image = match crate::data::asset_for(&p.image_key) {
                Some(src) => rsx! {
                    img {
                        class: "project-big-image-container",
                        src: src,
                        alt: "Screenshot of {p.title}"
                    }
                },
                None => rsx! {
                    div {
                        class: "project-big-image-container aspect-[16/10]",
                        aria_hidden: "true"
                    }
                },
            };

            rsx! {
                document::Title { "{p.title} — denizhoroz" }

                div {
                    id: "work-page",
                    class: "page-block",

                    h1 {
                        class: "block-subtitle",
                        "{p.title}"
                    }

                    {image}

                    p {
                        class: "block-desc zen-content",
                        "{p.description}"
                    }

                    div {
                        class: "tech-list",
                        for t in p.tech.iter() {
                            span { class: "tech-tag", key: "{t}", "{t}" }
                        }
                    }

                    a {
                        class: "block-desc button",
                        href: "{p.gitlink}",
                        img { src: GITHUB_ICON, alt: "", class: "social-icon"}
                        "Project GitHub Link"
                    }
                }
            }
        }
    };

    // The lg:pt-60 wrapper that used to sit here cleared the old fixed left rail
    // -- see the note in works/list.rs. The navbar is a top bar at every width
    // now and body's padding-top clears it, so the wrapper had nothing left to do.
    body
}
