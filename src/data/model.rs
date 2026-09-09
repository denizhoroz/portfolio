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

    /// ISO `YYYY-MM-DD`, as written in the content file. The detail page
    /// formats it for display; the stored form stays sortable and unambiguous.
    pub finish_date: String,

    /// The project's markdown body, already rendered to HTML.
    ///
    /// Rendered by `build.rs` rather than at runtime: the source never changes
    /// after a build, so parsing it in the browser would ship a markdown parser
    /// in the wasm bundle to produce a constant. Empty means the project has no
    /// body, and the detail page renders nothing.
    pub markdown_html: String,
}
