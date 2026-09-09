use dioxus::prelude::*;

use crate::data::whoami;

// AboutMe section
#[component]
pub fn AboutMe() -> Element {
    // Rendered from content/personal/whoami.md by build.rs, never by a user:
    // the source is the author's own and is compiled in from the repository.
    let prose = whoami();

    rsx! {
        div {
            id: "aboutme",
            class: "page-block page-section",

            div {
                class: "zen-content",
                h2 {
                    class: "block-title",
                    "who am i?"
                }

                // The prose arrives as bare <p> -- pulldown-cmark emits no
                // classes, and these paragraphs are no longer elements the rsx
                // can reach. .block-desc rides on the container instead, which
                // works because all three of its rules are inherited
                // properties: font-size, hyphens and text-align.
                //
                // A second .zen-content, not a bare div: it puts the same 30px
                // between the paragraphs that the outer one puts under the
                // heading, which is the gap they had as siblings of the <h2>.
                div {
                    class: "zen-content block-desc",
                    dangerous_inner_html: "{prose}"
                }
            }
        }
    }
}
