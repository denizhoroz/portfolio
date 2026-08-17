use dioxus::prelude::*;
use crate::data::projects;

#[component]
pub fn WorkPage(slug: String) -> Element {
    let project = projects().into_iter().find(|p| p.slug == slug);

    match project {
        Some(p) => rsx! {
            div {
                id: "work-page",
                class: "page-block",
                style: "gap: 20px;",
                
                h1 { 
                    class: "block-subtitle",
                    "{p.title}" 
                }
                
                img { 
                    class: "project-big-image-container",
                    src: p.image 
                }
                
                p { 
                    class: "block-desc zen-content",
                    "{p.description}" 
                }
                
                div {
                    class: "tech-list",
                    for t in p.tech.iter() {
                        span { class: "tech-tag", key: "{t}", "{t}" }
                    }
                }
            }
        },
        None => rsx! { "Project not found" },
    }
}