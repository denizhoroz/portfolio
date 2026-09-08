use super::{Project, static_source};

// The swap point. Everything above the UI talks to these two functions and
// nothing else, so moving to a real database means rewriting these bodies and
// touching no component. They stay `async` for that reason -- the day they do
// real I/O, no caller changes.

/// All projects, ordered by [`Project::position`] ascending.
pub async fn fetch_projects() -> Vec<Project> {
    static_source::all().await
}

/// One project by slug. `None` means no such project -- a 404, which the UI
/// renders differently from "still loading".
pub async fn fetch_project(slug: &str) -> Option<Project> {
    static_source::by_slug(slug).await
}
