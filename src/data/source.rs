use super::{static_source, Project};

/// Why a fetch failed.
///
/// Two variants is all the UI needs: something a retry could fix, and something
/// it could not. Widen this when a real transport exists.
// Nothing constructs these yet: the static source cannot fail. They exist for
// the source that replaces it, and the UI already branches on them. Drop the
// allow when a real transport lands.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum DataError {
    /// Transport failure -- offline, timeout, 5xx. Worth retrying.
    Unavailable(String),
    /// The source answered, but the payload was not what we expected.
    /// Retrying will not help; this is a bug or bad data.
    Malformed(String),
}

impl DataError {
    /// Whether to offer the user a retry button.
    pub fn is_retryable(&self) -> bool {
        matches!(self, DataError::Unavailable(_))
    }
}

impl std::fmt::Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataError::Unavailable(m) => write!(f, "{m}"),
            DataError::Malformed(m) => write!(f, "{m}"),
        }
    }
}

// ---------------------------------------------------------------------------
// The swap point.
//
// Everything above the UI talks to these two functions and nothing else. Moving
// to Turso means rewriting these bodies (or gating them behind a cfg) -- no
// component changes. They are async and fallible today precisely so that the
// day they start doing real I/O, no caller has to be touched.
// ---------------------------------------------------------------------------

/// All projects, ordered by [`Project::position`] ascending.
pub async fn fetch_projects() -> Result<Vec<Project>, DataError> {
    static_source::all().await
}

/// One project by slug. `Ok(None)` means "no such project" -- a 404, not an
/// error. Keep the two distinct: the UI renders them very differently.
pub async fn fetch_project(slug: &str) -> Result<Option<Project>, DataError> {
    static_source::by_slug(slug).await
}
