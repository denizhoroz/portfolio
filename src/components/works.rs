use dioxus::prelude::*;
use crate::{ProjectBox, Route, data::{ProjectBoxProps, projects}};

#[component]
pub fn MyWorksPage() -> Element {
    rsx! {
        div {
            class: "page-block",
            
            h1 {
                class: "block-title",
                style: "margin: 10px;",
                "all my works"
            }

            div {
                class: "project-container",


                for p in &projects() {
                    Link {
                        to: Route::WorkPage { slug: p.slug.clone() },
                        key: "{p.slug}",
                        { ProjectBox(ProjectBoxProps { ..p.clone() }) }
                    }
                }
            }
        }
    }
}