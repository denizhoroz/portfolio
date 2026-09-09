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

/// A row-shaped placeholder.
///
/// Reuses `.article-row` for the same reason `SkeletonCard` reuses
/// `.projectbox`: it occupies exactly the height a real row will, which is what
/// stops the list jumping when data lands.
#[component]
pub fn SkeletonRow() -> Element {
    rsx! {
        div {
            class: "article-row",
            aria_hidden: "true",

            div {
                class: "article-row-head",
                div { class: "skeleton h-7 w-2/3" }
                div { class: "skeleton h-4 w-20 shrink-0" }
            }
            div { class: "skeleton h-4 w-full" }
            div { class: "skeleton h-4 w-11/12" }

            div {
                class: "tech-list justify-start",
                div { class: "skeleton h-6 w-16 rounded-full" }
                div { class: "skeleton h-6 w-20 rounded-full" }
            }
        }
    }
}

/// A full column of row placeholders, in the same container the real rows use.
#[component]
pub fn SkeletonRowList(count: usize) -> Element {
    rsx! {
        p { class: "sr-only", aria_live: "polite", "Loading articles" }
        div {
            class: "article-container",
            for i in 0..count {
                SkeletonRow { key: "{i}" }
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

            div { class: "skeleton aspect-[16/10] w-full" }

            // Title left, date right -- the same row the loaded page renders.
            div {
                class: "flex w-full flex-col gap-2 sm:flex-row sm:items-start sm:justify-between sm:gap-4",
                div { class: "skeleton h-10 w-72 max-w-full" }
                div { class: "skeleton h-4 w-24 shrink-0" }
            }

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

/// Placeholder for a single article page.
///
/// Its own component rather than `SkeletonDetail` reused: that one opens with
/// the 16/10 image box a project page carries, and an article page has none, so
/// borrowing it would flash a large empty frame and then collapse it when the
/// text arrived.
#[component]
pub fn SkeletonArticleDetail() -> Element {
    rsx! {
        div {
            class: "page-block w-full gap-8",
            aria_hidden: "true",

            // Title left, date right -- the same row the loaded page renders.
            div {
                class: "flex w-full flex-col gap-2 sm:flex-row sm:items-start sm:justify-between sm:gap-4",
                div { class: "skeleton h-10 w-72 max-w-full" }
                div { class: "skeleton h-4 w-24 shrink-0" }
            }

            div {
                class: "zen-content w-full",
                div { class: "skeleton h-4 w-full" }
                div { class: "skeleton h-4 w-11/12" }
                div { class: "skeleton h-4 w-4/5" }
            }

            div {
                class: "tech-list",
                div { class: "skeleton h-6 w-16 rounded-full" }
                div { class: "skeleton h-6 w-20 rounded-full" }
            }
        }
        p { class: "sr-only", aria_live: "polite", "Loading article" }
    }
}
