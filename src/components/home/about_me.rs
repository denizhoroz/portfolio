use dioxus::prelude::*;

// AboutMe section
#[component]
pub fn AboutMe() -> Element {
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
                // Both of these were <h3>. They are prose, not headings.
                p {
                    class: "block-desc",
                    "Hello, I'm Deniz. I develop helpful apps to solve people's problems."
                }
                p {
                    class: "block-desc",
                    "I've always wanted to learn more about technology and since I've started my engineering journey in 2022, I am constantly trying to learn new things and apply the things I've learned by making new projects."
                }
            }
        }
    }
}
