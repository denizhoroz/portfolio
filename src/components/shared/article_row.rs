use dioxus::prelude::*;

use crate::{data::Article, format_date};

// The props type is deliberately not the data model, for the same reason as
// `ProjectBoxProps`: `Article` must be able to arrive from a database and
// `Asset` cannot.
#[derive(Props, Clone, PartialEq)]
pub struct ArticleRowProps {
    pub article: Article,
}

/// One article, as a full-width horizontal row.
///
/// The projects' counterpart is `ProjectBox`, a card in a grid. An article is a
/// row instead because its title and blurb are sentences rather than labels: a
/// 312px card column would wrap both to five or six lines, while a row gives
/// the text the whole measure.
///
/// No image, by design -- an article is text, so there is nothing to preview
/// and nothing to reserve space for.
#[component]
pub fn ArticleRow(props: ArticleRowProps) -> Element {
    let art = &props.article;

    rsx! {
        div {
            class: "article-row",

            // Title and date share a line, the date pushed to the right edge.
            // That is one row of height rather than two, which is most of what
            // keeps this box short.
            div {
                class: "article-row-head",

                h3 { class: "min-w-0 text-xl font-bold text-strong", "{art.title}" }

                // <time> so the machine-readable ISO value survives even though
                // the text beside it is the human form.
                time {
                    class: "project-date shrink-0 whitespace-nowrap",
                    datetime: "{art.publish_date}",
                    "{format_date(&art.publish_date)}"
                }
            }

            // font-content, unlike `ProjectBox`'s blurb: a project's is a
            // description written about it, but this is a sentence of the
            // article, so it is set in the same face the body will be.
            //
            // Clamped so a long blurb cannot stretch one row out of step with
            // the others; the full text is on the article's own page.
            p {
                class: "font-content line-clamp-2 hyphens-auto text-justify",
                "{art.description}"
            }

            // Hashtags, not a stack list -- an article is tagged by subject.
            // The pill itself is `.tech-tag`, reused rather than duplicated:
            // the two carry different meanings but the same visual weight, and
            // a second class with identical rules would only drift.
            div {
                class: "tech-list justify-start",
                for t in art.tags.iter() {
                    span { class: "tech-tag", key: "{t}", "#{t}" }
                }
            }
        }
    }
}
