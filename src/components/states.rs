use dioxus::prelude::*;

// Shared loading / error UI for the async data seam. Kept in one place so the
// three call sites (Home, /works, /works/:slug) cannot drift apart.

/// A card-shaped placeholder.
///
/// Reuses `.projectbox`, so it occupies exactly the height a real card will --
/// `min-h-[26rem]` plus the same padding and gaps. That is what stops the grid
/// jumping when data lands. Hidden from screen readers; the live region on the
/// grid announces the load instead.
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
            // max-w-3xl removed with the one on .project-big-image-container --
            // a placeholder narrower than the image it stands in for would make
            // the layout jump when the real one lands.
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

/// Failure state.
///
/// `role="alert"` so a screen reader is told without having to find it, and a
/// retry is offered only when retrying could actually help -- a malformed
/// payload will be just as malformed the second time.
#[component]
pub fn DataErrorBlock(
    title: String,
    detail: String,
    retryable: bool,
    on_retry: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div {
            class: "zen-content items-center",
            role: "alert",

            p { class: "block-desc", "{title}" }
            p { class: "text-sm text-subtle", "{detail}" }

            if retryable {
                button {
                    class: "button",
                    onclick: move |e| on_retry.call(e),
                    "Try again"
                }
            }
        }
    }
}
