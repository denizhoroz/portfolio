/// A project, as plain data.
///
/// Deliberately free of `Asset` and `#[derive(Props)]`: this type must be able
/// to arrive from a database, and `asset!()` is compile-time only. The render
/// contract lives in `ProjectBoxProps` instead.
#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub tech: Vec<String>,
    pub gitlink: String,

    /// A key, not a path -- resolved to a bundled asset by
    /// [`crate::data::asset_for`] so images stay compile-time assets.
    pub image_key: String,

    /// Display order, ascending. Mirrors `ORDER BY position` later.
    pub position: i32,
}
