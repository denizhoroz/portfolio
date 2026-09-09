use dioxus::prelude::*;

use crate::{AboutMe, Footer, Hero, Works, ReachMe};

/// Home page -- composition only; each section owns its own file.
#[component]
pub fn Home() -> Element {
    rsx! {
        document::Title { "denizhoroz" }

        // No gap or top padding: every child is a full-height .page-section, so
        // the section height IS the rhythm. A gap would show a strip of the next
        // section at the bottom of each screen.
        div {
            class: "flex flex-col",
            Hero {}
            AboutMe {}
            Works {}
            ReachMe {}
        }

        div {
            class: "pt-[clamp(4rem,12vw,10rem)]",
            Footer {}
        }
    }
}
