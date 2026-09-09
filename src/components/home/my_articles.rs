use dioxus::prelude::*;

use crate::{ Route };

/// How many projects the home page previews. The rest are on /works.
const HOME_PROJECT_COUNT: usize = 3;

#[component]
pub fn Articles() -> Element {
    rsx! {
        div {
            id: "myarticles",
            // Below lg this section is taller than one screen (3 cards at
            // min-h-[28rem], 1 column under 640px and 2 under 1024px), so
            // .page-section is a minimum here, not the actual height.
            class: "page-block page-section",
            div {
                class: "flex w-full flex-col items-center justify-center gap-5",
                h2 { class: "block-title", "latest articles" }

                // ProjectGrid {
                //     skeleton_count: HOME_PROJECT_COUNT,
                //     limit: HOME_PROJECT_COUNT,
                //     empty: "Nothing here yet.",
                // }

                Link { class: "block-desc button", to: Route::WorksPage {}, "see all articles" }
            }
        }
    }
}