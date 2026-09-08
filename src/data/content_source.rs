// `asset!()` is used by the generated code below.
use dioxus::prelude::*;

use super::Project;

// The content directory, compiled in.
//
// There is no filesystem in a wasm bundle, so `content/projects/` is read at
// build time by `build.rs` and emitted as the `projects()` constructor included
// below. Adding a project means adding its folder -- nothing here changes.
include!(concat!(env!("OUT_DIR"), "/content_generated.rs"));

pub async fn all() -> Vec<Project> {
    // Already sorted by id in build.rs, but sorted again here so the ordering
    // is a property of this function rather than of the generator.
    let mut list = projects();
    list.sort_by_key(|p| p.id);
    list
}

pub async fn by_slug(slug: &str) -> Option<Project> {
    projects().into_iter().find(|p| p.slug == slug)
}
