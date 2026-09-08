/// A project, as plain data.
///
/// Deliberately free of `Asset` and `#[derive(Props)]`: this type must be able
/// to arrive from a database, and `asset!()` is compile-time only. The render
/// contract lives in `ProjectBoxProps` instead.
///
/// The fields mirror `content/projects/<id>-<slug>/<id>-<slug>.json` one for
/// one -- `build.rs` builds these values straight from those files.
#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    /// Stable identity and display order, ascending. Mirrors `ORDER BY id`
    /// later, and names the content folder on disk.
    pub id: u32,

    pub slug: String,
    pub title: String,
    pub description: String,
    pub tech: Vec<String>,
    pub gitlink: String,

    /// The bundled URL of the project's image, already resolved.
    ///
    /// A `String`, not an `Asset`: `asset!()` is compile-time only and this
    /// type must be able to arrive from a database. `build.rs` writes the
    /// `asset!()` literal for `content/projects/<id>-<slug>/<image_key>` and
    /// stores what it resolves to. `None` means the file is missing -- the
    /// components keep the image box and render no `<img>`.
    pub image_src: Option<String>,

    /// ISO `YYYY-MM-DD`. Carried from the content file; not rendered yet.
    #[allow(dead_code)]
    pub finish_date: String,

    /// Filename of the project's markdown body, inside its content folder.
    /// Carried from the content file; not rendered yet.
    #[allow(dead_code)]
    pub markdown_key: String,
}
