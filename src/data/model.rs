/// A project, as plain data.
///
/// Deliberately free of both `Asset` and `#[derive(Props)]`. This type has to be
/// able to arrive from a database later, and `Asset` is produced by the
/// compile-time `asset!()` macro -- it cannot come off a wire. The rendering
/// contract lives in `ProjectBoxProps` next to the component instead.
#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub tech: Vec<String>,
    pub gitlink: String,

    /// Resolved to a bundled asset by [`crate::data::asset_for`]. A key, not a
    /// path or a URL, so the images stay compile-time assets in this repo.
    pub image_key: String,

    /// Display order, ascending. Mirrors `ORDER BY position` in a future
    /// database; today it just makes the old `vec!` ordering explicit.
    pub position: i32,
}
