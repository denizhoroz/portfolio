use dioxus::prelude::*;

use crate::{Route, SkeletonDetail, data::fetch_project};

const GITHUB_ICON: Asset = asset!("/assets/icons/github.svg");

const MONTHS: [&str; 12] = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December",
];

/// `2025-06-14` -> `14 June 2025`.
///
/// The stored form is ISO because it sorts and cannot be misread as US order;
/// this is the display form. Anything that does not parse is shown as written
/// rather than dropped -- a visible odd date is a bug report, a missing one is
/// silence.
fn format_date(iso: &str) -> String {
    let mut parts = iso.split('-');
    let (Some(y), Some(m), Some(d), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return iso.to_string();
    };

    let (Ok(month), Ok(day)) = (m.parse::<usize>(), d.parse::<u32>()) else {
        return iso.to_string();
    };

    match MONTHS.get(month.wrapping_sub(1)) {
        Some(name) => format!("{day} {name} {y}"),
        None => iso.to_string(),
    }
}

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

            // Rendered by build.rs, never by a user: see `Project::markdown_html`.
            let markdown = (!p.markdown_html.is_empty()).then(|| {
                rsx! {
                    hr { class: "section-rule" }

                    div {
                        class: "markdown-body",
                        dangerous_inner_html: "{p.markdown_html}"
                    }
                }
            });

            rsx! {

                div {
                    class: "page-block gap-8",

                    h1 { class: "block-subtitle", "{p.title}" }

                    // <time> so the machine-readable ISO value survives even
                    // though the text beside it is the human form.
                    time {
                        class: "project-date",
                        datetime: "{p.finish_date}",
                        "{format_date(&p.finish_date)}"
                    }

                    {image}

                    // text-center beats .block-desc's text-justify: utilities
                    // are layered after components.
                    p { class: "block-desc zen-content text-center", "{p.description}" }

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
                        "GitHub Link"
                    }

                    {markdown}
                }
            }
        }
    };

    body
}
