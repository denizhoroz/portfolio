use dioxus::prelude::*;

// Footer section
#[component]
pub fn Footer() -> Element {
    rsx! {
        // Two elements, on purpose:
        //
        //   outer  full-bleed, and the only thing it does is draw the rule, so
        //          it spans the whole page like the navbar's border-b rather
        //          than stopping at the shared measure.
        //   inner  .page-block, so the text still obeys the measure like every
        //          other region.
        //
        // min-h-[var(--bar-height)] is the same variable .navbar-inner uses, so
        // the footer is exactly as tall as the navbar -- 60px of content plus the
        // 1px rule, at both ends of the page. It replaces a hand-picked py-8:
        // that centred the text too, but only by coincidence of the numbers, and
        // it would have drifted the moment the navbar's height changed.
        // .page-block already supplies flex/justify-center, so the text centres
        // itself in that height with no extra rule.
        div {
            class: "border-t border-edge",
            div {
                class: "page-block min-h-[var(--bar-height)] text-subtle",
                "denizhoroz - 2026"
            }
        }
    }
}
