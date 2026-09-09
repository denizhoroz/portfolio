//! Turns `content/` into Rust at build time.
//!
//! The site is a wasm bundle: there is no filesystem at runtime, so content
//! cannot be read when a page loads. It is read here instead and emitted as
//! plain `Vec<Project>` / `Vec<Article>` constructors, which means no serde in
//! the shipped binary and a malformed or duplicated entry fails the build
//! rather than the page.
//!
//! Discovery is by directory scan, so adding `content/<kind>/<id>-<slug>/` with
//! its JSON and image is the whole job -- no list to update here or anywhere
//! else. Images ship through `asset!()`, which needs a literal path, so those
//! literals are written here too.
//!
//! `content/personal/` is the exception to the scan: it holds the site's
//! one-off prose, a file per fixed slot on a page, so each is read by name --
//! see `read_whoami`.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use pulldown_cmark::{Options, Parser, html};
use serde::Deserialize;

/// One `content/projects/<id>-<slug>/<id>-<slug>.json` file.
///
/// Mirrors `crate::data::Project`; kept separate because that type lives in the
/// crate being built and is not reachable from here.
#[derive(Deserialize)]
struct ProjectFile {
    id: u32,
    slug: String,
    title: String,
    description: String,
    image_key: String,
    gitlink: String,
    tech: Vec<String>,
    finish_date: String,
    markdown_key: String,

    /// Crate-root-relative path to `image_key` inside the project's folder,
    /// filled in after the file is read. `None` when the image is missing.
    #[serde(skip)]
    image_path: Option<String>,

    /// `markdown_key` rendered to HTML, filled in after the file is read.
    /// Empty when the file is missing or has no content -- the detail page
    /// renders nothing rather than an empty box.
    #[serde(skip)]
    markdown_html: String,
}

/// One `content/articles/<id>-<slug>/<id>-<slug>.json` file.
///
/// Deliberately its own type rather than a reuse of `ProjectFile`: an article
/// has no repository to link and is dated by publication rather than
/// completion, and collapsing the two would leave every article carrying an
/// empty `gitlink` for a field it does not have.
#[derive(Deserialize)]
struct ArticleFile {
    id: u32,
    slug: String,
    title: String,
    description: String,
    tags: Vec<String>,
    publish_date: String,
    markdown_key: String,

    #[serde(skip)]
    markdown_html: String,
}

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let content = root.join("content");

    // The directory itself, so a newly added or removed folder is picked up;
    // each file, so an edit to one is.
    println!("cargo:rerun-if-changed=content");
    println!("cargo:rerun-if-changed=build.rs");

    let mut projects = read_projects(&content.join("projects"));
    projects.sort_by_key(|p| p.id);
    check_unique("project", projects.iter().map(|p| (p.id, p.slug.as_str())));

    let mut articles = read_articles(&content.join("articles"));
    articles.sort_by_key(|a| a.id);
    check_unique("article", articles.iter().map(|a| (a.id, a.slug.as_str())));

    let whoami = read_whoami(&content.join("personal"));

    let out = Path::new(&std::env::var("OUT_DIR").expect("OUT_DIR is set by cargo"))
        .join("content_generated.rs");

    let mut generated = render_projects(&projects);
    generated.push_str(&render_articles(&articles));
    generated.push_str(&render_whoami(&whoami));

    fs::write(&out, generated).expect("failed to write generated content");
}

/// One entry found by the directory scan: its folder name, its path, and the
/// contents of the JSON file named after the folder.
struct Entry {
    folder: String,
    path: PathBuf,
    raw: String,
}

/// Walks `content/<kind>/`, enforcing the `<id>-<slug>/<id>-<slug>.json` naming
/// rule. Shared by both content types so that rule cannot drift between them.
fn scan(dir: &Path) -> Vec<Entry> {
    // A missing directory is an empty section, not a broken build: the grid and
    // the article list both already have an "empty" state and render it.
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => {
            println!(
                "cargo:warning=no {} directory; building with none",
                dir.display()
            );
            return Vec::new();
        }
    };

    let mut found = Vec::new();

    for entry in entries {
        let entry = entry.expect("failed to read content entry");
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        // <id>-<slug>/<id>-<slug>.json -- the file is named after its folder.
        let folder = path
            .file_name()
            .and_then(|n| n.to_str())
            .expect("content folder name is not valid UTF-8")
            .to_string();
        let json_path = path.join(format!("{folder}.json"));

        if !json_path.exists() {
            panic!(
                "{}/ has no {folder}.json -- \
                 the JSON file must be named after its folder",
                path.display()
            );
        }

        let raw = fs::read_to_string(&json_path)
            .unwrap_or_else(|e| panic!("failed to read {}: {e}", json_path.display()));

        found.push(Entry { folder, path, raw });
    }

    found
}

/// The folder name is derived data; if it drifts from the JSON, one of the two
/// is wrong and the URL would not match the file on disk.
fn check_folder_name(folder: &str, id: u32, slug: &str) {
    let expected = format!("{id}-{slug}");
    if folder != expected {
        panic!(
            "content folder {folder}/ declares id {id} and slug \"{slug}\", \
             so the folder should be named {expected}"
        );
    }
}

/// Resolves `image_key` to a crate-root-relative path for `asset!()`.
///
/// Projects only -- an article is text, and its row carries no image box.
///
/// A missing image is not fatal: the card and the detail page both keep their
/// image box -- it carries the aspect-ratio that reserves the space -- and
/// render no `<img>`, which beats a broken-image icon.
fn resolve_image(kind: &str, folder: &str, dir: &Path, image_key: &str) -> Option<String> {
    if dir.join(image_key).exists() {
        Some(format!("/content/{kind}/{folder}/{image_key}"))
    } else {
        println!("cargo:warning=content/{kind}/{folder}/{image_key} is missing");
        None
    }
}

/// Reads and renders `markdown_key`.
///
/// The body is rendered here rather than in the app: pulldown-cmark would
/// otherwise ship in the wasm bundle to parse text that never changes after a
/// build. A missing file is not fatal -- an empty body renders as no body.
fn resolve_markdown(kind: &str, folder: &str, dir: &Path, markdown_key: &str) -> String {
    let path = dir.join(markdown_key);
    if !path.exists() {
        println!("cargo:warning=content/{kind}/{folder}/{markdown_key} is missing");
        return String::new();
    }

    let md = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
    render_markdown(&md)
}

fn read_projects(dir: &Path) -> Vec<ProjectFile> {
    let mut projects = Vec::new();

    for entry in scan(dir) {
        let mut project: ProjectFile = serde_json::from_str(&entry.raw)
            .unwrap_or_else(|e| panic!("invalid JSON in {}: {e}", entry.folder));

        check_folder_name(&entry.folder, project.id, &project.slug);

        project.markdown_html =
            resolve_markdown("projects", &entry.folder, &entry.path, &project.markdown_key);
        project.image_path =
            resolve_image("projects", &entry.folder, &entry.path, &project.image_key);

        projects.push(project);
    }

    projects
}

/// Reads and renders `content/personal/whoami.md`, the prose behind Home's
/// "who am i?".
///
/// Deliberately not a directory scan like the two above: there is exactly one
/// of these, and it carries no id, slug, date or image -- so the
/// `<id>-<slug>/<id>-<slug>.json` rule and the JSON file it names would be
/// ceremony around two paragraphs of prose. The filename is a literal here for
/// the same reason: nothing chooses between several of these.
///
/// A missing file is not fatal, as with a write-up: the section keeps its
/// heading and renders no prose under it.
fn read_whoami(dir: &Path) -> String {
    let path = dir.join("whoami.md");
    if !path.exists() {
        println!("cargo:warning=content/personal/whoami.md is missing");
        return String::new();
    }

    let md = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
    render_markdown(&md)
}

fn read_articles(dir: &Path) -> Vec<ArticleFile> {
    let mut articles = Vec::new();

    for entry in scan(dir) {
        let mut article: ArticleFile = serde_json::from_str(&entry.raw)
            .unwrap_or_else(|e| panic!("invalid JSON in {}: {e}", entry.folder));

        check_folder_name(&entry.folder, article.id, &article.slug);

        article.markdown_html =
            resolve_markdown("articles", &entry.folder, &entry.path, &article.markdown_key);

        articles.push(article);
    }

    articles
}

/// CommonMark plus the table, footnote and strikethrough extensions -- the
/// parts of "GitHub markdown" a write-up actually reaches for.
///
/// Raw HTML in a source file is passed through as written. The content is the
/// author's own and is compiled in from the repository, so there is no
/// untrusted input here; the output goes to `dangerous_inner_html`.
fn render_markdown(md: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let mut out = String::new();
    html::push_html(&mut out, Parser::new_ext(md, options));
    out
}

/// Ids order the list and slugs are the URL, so a duplicate of either silently
/// hides an entry. Fail the build instead.
fn check_unique<'a>(kind: &str, entries: impl Iterator<Item = (u32, &'a str)>) {
    let mut ids: HashMap<u32, &str> = HashMap::new();
    let mut slugs: HashMap<&str, u32> = HashMap::new();

    for (id, slug) in entries {
        if let Some(other) = ids.insert(id, slug) {
            panic!("duplicate {kind} id {id}: \"{other}\" and \"{slug}\"");
        }
        if let Some(other) = slugs.insert(slug, id) {
            panic!("duplicate {kind} slug \"{slug}\": ids {other} and {id}");
        }
    }
}

/// `asset!()` hashes and bundles the file and needs a literal path, so the
/// literal is written here. `.to_string()` keeps the model itself free of
/// `Asset`, which is compile-time only and could never come from a database.
fn image_expr(image_path: &Option<String>) -> String {
    match image_path {
        Some(path) => format!("Some(asset!({path:?}).to_string())"),
        None => "None".to_string(),
    }
}

/// Emits the body of `content_source::projects()`.
///
/// Every string goes through `{:?}`, which is Rust's own string-literal
/// escaping -- quotes and backslashes in content cannot break the generated
/// source.
fn render_projects(projects: &[ProjectFile]) -> String {
    let mut out = String::from(
        "// @generated by build.rs from content/ -- do not edit.\n\
         fn projects() -> Vec<Project> {\n    vec![\n",
    );

    for p in projects {
        let tech = p
            .tech
            .iter()
            .map(|t| format!("{t:?}.to_string()"))
            .collect::<Vec<_>>()
            .join(", ");

        out.push_str(&format!(
            "        Project {{
            id: {id},
            slug: {slug:?}.to_string(),
            title: {title:?}.to_string(),
            description: {description:?}.to_string(),
            image_src: {image_src},
            gitlink: {gitlink:?}.to_string(),
            tech: vec![{tech}],
            finish_date: {finish_date:?}.to_string(),
            markdown_html: {markdown_html:?}.to_string(),
        }},\n",
            id = p.id,
            slug = p.slug,
            title = p.title,
            description = p.description,
            image_src = image_expr(&p.image_path),
            gitlink = p.gitlink,
            tech = tech,
            finish_date = p.finish_date,
            markdown_html = p.markdown_html,
        ));
    }

    out.push_str("    ]\n}\n");
    out
}

/// Emits the body of `content_source::articles()`.
fn render_articles(articles: &[ArticleFile]) -> String {
    let mut out = String::from("\nfn articles() -> Vec<Article> {\n    vec![\n");

    for a in articles {
        let tags = a
            .tags
            .iter()
            .map(|t| format!("{t:?}.to_string()"))
            .collect::<Vec<_>>()
            .join(", ");

        out.push_str(&format!(
            "        Article {{
            id: {id},
            slug: {slug:?}.to_string(),
            title: {title:?}.to_string(),
            description: {description:?}.to_string(),
            tags: vec![{tags}],
            publish_date: {publish_date:?}.to_string(),
            markdown_html: {markdown_html:?}.to_string(),
        }},\n",
            id = a.id,
            slug = a.slug,
            title = a.title,
            description = a.description,
            tags = tags,
            publish_date = a.publish_date,
            markdown_html = a.markdown_html,
        ));
    }

    out.push_str("    ]\n}\n");
    out
}

/// Emits `content_source::whoami_html()`.
///
/// A `&'static str` rather than the `Vec` constructors above: there is one of
/// these and it never varies, so there is nothing to allocate and no owner to
/// hand it to.
fn render_whoami(html: &str) -> String {
    format!("
fn whoami_html() -> &'static str {{
    {html:?}
}}
")
}
