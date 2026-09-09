// `asset!()` is used by the generated code below.
use dioxus::prelude::*;

use super::{Article, Project};

// The content directory, compiled in.
//
// There is no filesystem in a wasm bundle, so `content/projects/` is read at
// build time by `build.rs` and emitted as the `projects()` constructor included
// below. Adding a project means adding its folder -- nothing here changes.
include!(concat!(env!("OUT_DIR"), "/content_generated.rs"));

pub async fn all() -> Vec<Project> {
    // Newest first, so that Home's `limit: 3` is the three most recent rather
    // than the three lowest ids. finish_date is ISO YYYY-MM-DD, so a string
    // compare is a date compare -- no parsing, and no date type in the bundle.
    // Descending id breaks a tie, which keeps the order stable when two
    // projects share a date.
    //
    // Sorted here rather than in build.rs so the ordering is a property of this
    // function rather than of the generator.
    let mut list = projects();
    list.sort_by(|a, b| b.finish_date.cmp(&a.finish_date).then(b.id.cmp(&a.id)));
    list
}

pub async fn by_slug(slug: &str) -> Option<Project> {
    projects().into_iter().find(|p| p.slug == slug)
}

pub async fn all_articles() -> Vec<Article> {
    // Newest first, for the reasons in `all` -- publish_date is the article's
    // equivalent of finish_date.
    let mut list = articles();
    list.sort_by(|a, b| b.publish_date.cmp(&a.publish_date).then(b.id.cmp(&a.id)));
    list
}

pub async fn article_by_slug(slug: &str) -> Option<Article> {
    articles().into_iter().find(|a| a.slug == slug)
}

/// Home's "who am i?" prose, as HTML.
///
/// Not `async` like the four above -- see `source::whoami`.
pub fn whoami() -> &'static str {
    whoami_html()
}
