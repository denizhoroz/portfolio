use dioxus::prelude::*;

use crate::{Route, SkeletonArticleDetail, data::fetch_article, format_date};

/// `/articles/:slug` -- one article, body included.
///
/// The projects' counterpart is `works::detail`, and the shape is the same:
/// resource, three states, then the rendered `markdown_html`. What is missing
/// against it is missing by design -- an article is text, so there is no cover
/// image to reserve space for and no repository to link.
#[component]
pub fn ArticlePage(slug: String) -> Element {
    // use_reactive! is load-bearing: use_resource captures its closure once, and
    // /articles/a -> /articles/b reuses this component with only the prop
    // swapped, so without it the future keeps fetching the slug it was first
    // built with.
    let article = use_resource(use_reactive!(|slug| async move {
        fetch_article(&slug).await
    }));
    let state = article.value();

    // Three states, not two: Some(None) is a real 404, None is "still asking".
    // Collapsing them flashes "Article not found" on every page load.
    // Bound to a local rather than returned directly: the match borrows the read
    // guard, and as a tail expression that borrow outlives `state`.
    let body = match &*state.read() {
        None => rsx! { SkeletonArticleDetail {} },

        Some(None) => rsx! {
            div {
                class: "page-block gap-6",

                h1 { class: "block-subtitle", "Article not found" }

                p {
                    class: "block-desc zen-content",
                    "There is no article at this address. It may have been renamed, or the link may be out of date."
                }
            }
        },

        Some(Some(a)) => {
            // Rendered by build.rs, never by a user: see `Article::markdown_html`.
            let markdown = (!a.markdown_html.is_empty()).then(|| {
                rsx! {
                    hr { class: "section-rule" }

                    div {
                        class: "markdown-body",
                        dangerous_inner_html: "{a.markdown_html}"
                    }
                }
            });

            rsx! {
                div {
                    class: "page-block gap-8",

                    // Title hard left, date hard right, on one line. Stacked
                    // below sm: .block-subtitle is clamp(2rem, 5vw, 2.5rem), so
                    // on a phone a row leaves the title wrapping to three lines
                    // against a date pinned to the edge.
                    div {
                        class: "flex w-full flex-col gap-2 text-left sm:flex-row sm:items-start sm:justify-between sm:gap-4",

                        h1 { class: "block-subtitle", "{a.title}" }

                        // <time> so the machine-readable ISO value survives even
                        // though the text beside it is the human form.
                        // shrink-0 + nowrap so the title takes the wrapping and
                        // the date stays whole on the right edge. items-start,
                        // not items-baseline: the title is clamp(2rem, 5vw,
                        // 2.5rem), so sharing its baseline drops the date to
                        // the bottom of the row.
                        time {
                            class: "project-date shrink-0 whitespace-nowrap",
                            datetime: "{a.publish_date}",
                            "{format_date(&a.publish_date)}"
                        }
                    }

                    // No text-center: .block-desc already carries text-justify
                    // (with hyphens-auto to keep the rag off), and a utility
                    // here would beat it -- utilities are layered after
                    // components.
                    p { class: "block-desc zen-content", "{a.description}" }

                    // Hashtags, not a stack list -- an article is tagged by
                    // subject. Same `#` prefix and same pill as `ArticleRow`,
                    // so a tag reads the same in the list and on the page.
                    div {
                        class: "tech-list",
                        for t in a.tags.iter() {
                            span { class: "tech-tag", key: "{t}", "#{t}" }
                        }
                    }

                    {markdown}
                }
            }
        }
    };

    rsx! {
        // Outside the match, in its own .page-block: the arms carry gap-6/gap-8,
        // which would add to the 20px below, and a go back that only appeared
        // once the fetch landed would push the whole page down on arrival.
        div {
            class: "page-block",

            Link { class: "block-desc button mb-[20px]", to: Route::ArticlesPage {}, "go back" }
        }

        {body}
    }
}
