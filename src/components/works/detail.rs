use dioxus::prelude::*;

use crate::{Breadcrumb, Crumb, Route, SkeletonDetail, data::fetch_project, format_date};

const GITHUB_ICON: Asset = asset!("/assets/icons/github.svg");

#[component]
pub fn WorkPage(slug: String) -> Element {
    // Cloned before use_reactive! consumes the prop below. Used as the trail's
    // last crumb when the fetch comes back empty: on a 404 the slug is the only
    // name this page has.
    let slug_label = slug.clone();

    // use_reactive! is load-bearing: use_resource captures its closure once, and
    // /works/a -> /works/b reuses this component with only the prop swapped, so
    // without it the future keeps fetching the slug it was first built with.
    let project = use_resource(use_reactive!(|slug| async move {
        fetch_project(&slug).await
    }));
    let state = project.value();

    // The trail's last crumb. Read in its own statement and owned rather than
    // borrowed, so the guard drops here: the trail renders outside the `body`
    // match below -- see the comment at the bottom of this function -- and so
    // cannot be built inside one of its arms.
    //
    // Some(None) falls back to the slug. On a 404 there is no title to show,
    // and the address is the only name the page has.
    let current: Option<String> = match &*state.read() {
        None => None,
        Some(None) => Some(slug_label.clone()),
        Some(Some(p)) => Some(p.title.clone()),
    };

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

                    {image}

                    // Title hard left, date hard right, on one line. Stacked
                    // below sm: .block-subtitle is clamp(2rem, 5vw, 2.5rem), so
                    // on a phone a row leaves the title wrapping to three lines
                    // against a date pinned to the edge.
                    div {
                        class: "flex w-full flex-col gap-2 text-left sm:flex-row sm:items-start sm:justify-between sm:gap-4",

                        h1 { class: "block-subtitle", "{p.title}" }

                        // <time> so the machine-readable ISO value survives even
                        // though the text beside it is the human form.
                        // shrink-0 + nowrap so the title takes the wrapping and
                        // the date stays whole on the right edge. items-start,
                        // not items-baseline: the title is clamp(2rem, 5vw,
                        // 2.5rem), so sharing its baseline drops the date to
                        // the bottom of the row.
                        time {
                            class: "project-date shrink-0 whitespace-nowrap",
                            datetime: "{p.finish_date}",
                            "{format_date(&p.finish_date)}"
                        }
                    }

                    // No text-center: .block-desc already carries text-justify
                    // (with hyphens-auto to keep the rag off), and a utility
                    // here would beat it -- utilities are layered after
                    // components.
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
                        "GitHub Link"
                    }

                    {markdown}
                }
            }
        }
    };

    rsx! {
        // Outside the match, in its own .page-block: the arms carry gap-6/gap-8,
        // which would add to the 20px below, and a trail that only appeared once
        // the fetch landed would push the whole page down on arrival.
        //
        // Only the last crumb waits on the fetch, and Crumb::pending holds its
        // width while it does, so the title arriving changes that crumb and
        // nothing else.
        div {
            class: "page-block",

            Breadcrumb {
                trail: vec![
                    Crumb::link("home", Route::Home {}),
                    Crumb::link("works", Route::WorksPage {}),
                    match current {
                        Some(title) => Crumb::current(title),
                        None => Crumb::pending(),
                    },
                ],
            }
        }

        {body}
    }
}
