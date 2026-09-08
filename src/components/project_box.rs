use dioxus::prelude::*;

use crate::data::{Project, asset_for};

// Small Project Box
// The props type is no longer the data model. `Project` has to be able to come
// off a wire later and `Asset` cannot, so the asset is resolved here, at render
// time, from the row's `image_key`.
#[derive(Props, Clone, PartialEq)]
pub struct ProjectBoxProps {
    pub project: Project,
}

#[component]
pub fn ProjectBox(props: ProjectBoxProps) -> Element {
    let proj = &props.project;

    // `asset_for` returns None only on an image_key we don't bundle. Keep the
    // box -- it carries the aspect-ratio that reserves the space -- but render
    // no <img>, rather than showing the browser's broken-image icon.
    let image = match asset_for(&proj.image_key) {
        Some(src) => rsx! {
            img {
                class: "image-portrait",
                src: src,
                alt: "Screenshot of {proj.title}"
            }
        },
        None => rsx! {
            div { class: "image-portrait", aria_hidden: "true" }
        },
    };

    rsx! {
        div {
            class: "projectbox",

            {image}

            // text-xl, not text-2xl: the shared 1024px measure makes the cards
            // ~312px instead of ~357px, and at 24px the longer titles wrapped to
            // three lines.
            h3 {
                class: "text-xl font-bold text-strong",
                "{proj.title}"
            }

            // Clamped so a long description can't push the card out of shape;
            // the full text is on the project's own page.
            // Justified like the rest of the body copy. hyphens-auto matters more
            // here than anywhere else -- at ~312px this is the narrowest measure
            // on the site, so it is where justification would otherwise open the
            // widest gaps between words.
            // clamp-6, not 4. Measured at the 312px card width: the longest
            // blurb wants 6 lines and three of the six were being cut off
            // mid-sentence. 6 is the measured maximum, not a guess -- the clamp
            // stays so that one very long description can never distort a row.
            p {
                class: "line-clamp-6 hyphens-auto text-justify",
                "{proj.description}"
            }

            // mt-auto pins the tags to the bottom regardless of description length
            div {
                class: "tech-list mt-auto",
                for t in proj.tech.iter() {
                    span { class: "tech-tag", key: "{t}", "{t}" }
                }
            }
        }
    }
}
