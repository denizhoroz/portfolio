use dioxus::prelude::*;

/// A card-shaped placeholder.
///
/// Reuses `.projectbox` so it occupies exactly the height a real card will,
/// which is what stops the grid jumping when data lands. Hidden from screen
/// readers; the live region on the grid announces the load instead.
#[component]
pub fn SkeletonCard() -> Element {
    rsx! {
        div {
            class: "projectbox",
            aria_hidden: "true",

            div { class: "skeleton aspect-[16/10] w-full" }
            div { class: "skeleton h-7 w-2/3" }
            div { class: "skeleton h-4 w-full" }
            div { class: "skeleton h-4 w-11/12" }
            div { class: "skeleton h-4 w-3/5" }

            div {
                class: "tech-list mt-auto",
                div { class: "skeleton h-6 w-16 rounded-full" }
                div { class: "skeleton h-6 w-20 rounded-full" }
                div { class: "skeleton h-6 w-14 rounded-full" }
            }
        }
    }
}

/// A full grid of placeholders, in the same container the real cards use.
#[component]
pub fn SkeletonGrid(count: usize) -> Element {
    rsx! {
        p { class: "sr-only", aria_live: "polite", "Loading projects" }
        div {
            class: "project-container",
            for i in 0..count {
                SkeletonCard { key: "{i}" }
            }
        }
    }
}

/// Placeholder for a single project page.
#[component]
pub fn SkeletonDetail() -> Element {
    rsx! {
        div {
            class: "page-block w-full gap-8",
            aria_hidden: "true",

            div { class: "skeleton h-10 w-72 max-w-full" }
            div { class: "skeleton aspect-[16/10] w-full" }

            div {
                class: "zen-content w-full",
                div { class: "skeleton h-4 w-full" }
                div { class: "skeleton h-4 w-11/12" }
                div { class: "skeleton h-4 w-4/5" }
            }
        }
        p { class: "sr-only", aria_live: "polite", "Loading project" }
    }
}
