use super::{Article, Project, content_source};

// The swap point. Everything above the UI talks to these functions and nothing
// else, so moving to a real database means rewriting these bodies and touching
// no component. The four that return content collections stay `async` for that
// reason -- the day they do real I/O, no caller changes.

/// All projects, newest [`Project::finish_date`] first.
pub async fn fetch_projects() -> Vec<Project> {
    content_source::all().await
}

/// One project by slug. `None` means no such project -- a 404, which the UI
/// renders differently from "still loading".
pub async fn fetch_project(slug: &str) -> Option<Project> {
    content_source::by_slug(slug).await
}

/// All articles, newest [`Article::publish_date`] first.
pub async fn fetch_articles() -> Vec<Article> {
    content_source::all_articles().await
}

/// One article by slug. `None` means no such article -- a 404, which the UI
/// renders differently from "still loading".
pub async fn fetch_article(slug: &str) -> Option<Article> {
    content_source::article_by_slug(slug).await
}

/// Home's "who am i?" prose, rendered from `content/personal/whoami.md` at
/// build time.
///
/// Sync, and the one exception to the rule above. This is a single string that
/// is in the bundle before the first paint, not a collection to query, so a
/// `use_resource` around it would buy a future database migration nothing and
/// cost the home page a loading state -- a blank gap under the heading on
/// every load, for text that is already there.
pub fn whoami() -> &'static str {
    content_source::whoami()
}
