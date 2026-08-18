use dioxus::prelude::*;
use crate::data::projects;

const GITHUB_ICON: Asset = asset!("/assets/icons/github.svg");

#[component]
pub fn WorkPage(slug: String) -> Element {
    let project = projects().into_iter().find(|p| p.slug == slug);

    match project {
        Some(p) => rsx! {
            div {
                id: "work-page",
                class: "page-block",
                
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

                a {
                    class: "block-desc button",
                    href: p.gitlink,
                    img { style: "width: 40px", src: GITHUB_ICON, alt: "GitHub", class: "social-icon"} 
                    "Project GitHub Link"
                }
            }
        },
        None => rsx! { "Project not found" },
    }
}