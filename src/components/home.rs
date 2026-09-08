use dioxus::prelude::*;

use crate::{AboutMe, Footer, Hero, MyWorks, ReachMe};

/// Home page.
///
/// Nothing but composition -- each section owns its own file, so this is the
/// place to reorder them or drop one in, and nowhere else needs to change.
#[component]
pub fn Home() -> Element {
    rsx! {
        document::Title { "denizhoroz" }

        // No gap and no top padding: every child is a full-height .page-section,
        // so the rhythm comes from the section height itself. A gap here would
        // show a strip of the next section at the bottom of each screen, and top
        // padding would push #hero past one screen.
        div {
            class: "flex flex-col",
            Hero {}
            AboutMe {}
            MyWorks {}
            ReachMe {}
        }

        div {
            class: "pt-[clamp(4rem,12vw,10rem)]",
            Footer {}
        }
    }
}
