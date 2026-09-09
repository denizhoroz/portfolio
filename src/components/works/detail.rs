use dioxus::prelude::*;

use crate::{Route, SkeletonDetail, data::fetch_project};

const GITHUB_ICON: Asset = asset!("/assets/icons/github.svg");

#[component]
pub fn WorkPage(slug: String) -> Element {
    // use_reactive! is load-bearing: use_resource captures its closure once, and
    // /works/a -> /works/b reuses this component with only the prop swapped, so
    // without it the future keeps fetching the slug it was first built with.
    let project = use_resource(use_reactive!(|slug| async move {
        fetch_project(&slug).await
    }));
    let state = project.value();

    // Three states, not two: Some(None) is a real 404, None is "still asking".
    // Collapsing them flashes "Project not found" on every page load.
    // Bound to a local rather than returned directly: the match borrows the read
    // guard, and as a tail expression that borrow outlives `state`.
    let body = match &*state.read() {
        None => rsx! { SkeletonDetail {} },

        Some(None) => rsx! {
            document::Title { "Project not found — denizhoroz" }

            div {
                class: "page-block gap-6",

                h1 { class: "block-subtitle", "Project not found" }

                p {
                    class: "block-desc zen-content",
                    "There is no project at this address. It may have been renamed, or the link may be out of date."
                }

                Link { class: "block-desc button", to: Route::WorksPage {}, "Back to all works" }
            }
        },

        Some(Some(p)) => {
            // image_src is None only when the project folder has no image file.
            // Keep the box -- it carries the aspect-ratio that reserves space --
            // and render no <img>, rather than a broken-image icon.
            let image = match &p.image_src {
                Some(src) => rsx! {
                    img {
                        class: "project-big-image-container",
                        src: "{src}",
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
                    class: "page-block gap-8",

                    h1 { class: "block-subtitle", "{p.title}" }

                    {image}

                    p { class: "block-desc zen-content", "{p.description}" }

                    div {
                        class: "tech-list",
                        for t in p.tech.iter() {
                            span { class: "tech-tag", key: "{t}", "{t}" }
                        }
                    }

                    a {
                        class: "block-desc button",
                        href: "{p.gitlink}",
                        img { src: GITHUB_ICON, alt: "", class: "social-icon" }
                        "Project GitHub Link"
                    }
                }
            }
        }
    };

    body
}
