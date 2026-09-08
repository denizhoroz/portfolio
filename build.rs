//! Turns `content/` into Rust at build time.
//!
//! The site is a wasm bundle: there is no filesystem at runtime, so content
//! cannot be read when a page loads. It is read here instead and emitted as a
//! plain `Vec<Project>` constructor, which means no serde in the shipped
//! binary and a malformed or duplicated entry fails the build rather than the
//! page.
//!
//! Discovery is by directory scan, so adding `content/projects/<id>-<slug>/`
//! with its JSON and image is the whole job -- no list to update here or
//! anywhere else. Images ship through `asset!()`, which needs a literal path,
//! so those literals are written here too.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

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
}

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let projects_dir = root.join("content").join("projects");

    // The directory itself, so a newly added or removed project folder is
    // picked up; each file, so an edit to one is.
    println!("cargo:rerun-if-changed=content");
    println!("cargo:rerun-if-changed=build.rs");

    let mut projects = read_projects(&projects_dir);
    projects.sort_by_key(|p| p.id);
    check_unique(&projects);

    let out = Path::new(&std::env::var("OUT_DIR").expect("OUT_DIR is set by cargo"))
        .join("content_generated.rs");
    fs::write(&out, render(&projects)).expect("failed to write generated content");
}

fn read_projects(dir: &Path) -> Vec<ProjectFile> {
    // A missing content/ directory is an empty site, not a broken build: the
    // grid already has an "empty" state and renders it.
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => {
            println!("cargo:warning=no content/projects directory; building with no projects");
            return Vec::new();
        }
    };

    let mut projects = Vec::new();

    for entry in entries {
        let entry = entry.expect("failed to read content/projects entry");
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        // <id>-<slug>/<id>-<slug>.json -- the file is named after its folder.
        let folder = path
            .file_name()
            .and_then(|n| n.to_str())
            .expect("project folder name is not valid UTF-8")
            .to_string();
        let json_path = path.join(format!("{folder}.json"));

        if !json_path.exists() {
            panic!(
                "content/projects/{folder}/ has no {folder}.json -- \
                 the JSON file must be named after its folder"
            );
        }

        let raw = fs::read_to_string(&json_path)
            .unwrap_or_else(|e| panic!("failed to read {}: {e}", json_path.display()));
        let mut project: ProjectFile = serde_json::from_str(&raw)
            .unwrap_or_else(|e| panic!("invalid JSON in {}: {e}", json_path.display()));

        // The folder name is derived data; if it drifts from the JSON, one of
        // the two is wrong and the URL would not match the file on disk.
        let expected = format!("{}-{}", project.id, project.slug);
        if folder != expected {
            panic!(
                "content/projects/{folder}/ declares id {} and slug \"{}\", \
                 so the folder should be named {expected}",
                project.id, project.slug
            );
        }

        // Not fatal: the markdown body is not rendered yet, and a missing file
        // should not stop a build.
        if !path.join(&project.markdown_key).exists() {
            println!(
                "cargo:warning=content/projects/{folder}/{} is missing",
                project.markdown_key
            );
        }

        // Also not fatal: the card and the detail page both keep their image
        // box -- it carries the aspect-ratio that reserves the space -- and
        // render no <img>, which beats a broken-image icon.
        project.image_path = if path.join(&project.image_key).exists() {
            Some(format!("/content/projects/{folder}/{}", project.image_key))
        } else {
            println!(
                "cargo:warning=content/projects/{folder}/{} is missing",
                project.image_key
            );
            None
        };

        projects.push(project);
    }

    projects
}

/// Ids order the grid and slugs are the URL, so a duplicate of either silently
/// hides a project. Fail the build instead.
fn check_unique(projects: &[ProjectFile]) {
    let mut ids: HashMap<u32, &str> = HashMap::new();
    let mut slugs: HashMap<&str, u32> = HashMap::new();

    for p in projects {
        if let Some(other) = ids.insert(p.id, &p.slug) {
            panic!("duplicate project id {}: \"{}\" and \"{}\"", p.id, other, p.slug);
        }
        if let Some(other) = slugs.insert(&p.slug, p.id) {
            panic!("duplicate project slug \"{}\": ids {} and {}", p.slug, other, p.id);
        }
    }
}

/// Emits the body of `content_source::projects()`.
///
/// Every string goes through `{:?}`, which is Rust's own string-literal
/// escaping -- quotes and backslashes in content cannot break the generated
/// source.
fn render(projects: &[ProjectFile]) -> String {
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

        // `asset!()` hashes and bundles the file and needs a literal path, so
        // the literal is baked in here. `.to_string()` keeps the model itself
        // free of `Asset`, which is compile-time only and could never come
        // from a database.
        let image_src = match &p.image_path {
            Some(path) => format!("Some(asset!({path:?}).to_string())"),
            None => "None".to_string(),
        };

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
            markdown_key: {markdown_key:?}.to_string(),
        }},\n",
            id = p.id,
            slug = p.slug,
            title = p.title,
            description = p.description,
            image_src = image_src,
            gitlink = p.gitlink,
            tech = tech,
            finish_date = p.finish_date,
            markdown_key = p.markdown_key,
        ));
    }

    out.push_str("    ]\n}\n");
    out
}
