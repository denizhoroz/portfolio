use dioxus::prelude::*;

// Only this section renders the social icons, so the assets live with it.
const GITHUB_ICON: Asset = asset!("/assets/icons/github.svg");
const LINKEDIN_ICON: Asset = asset!("/assets/icons/linkedin.svg");
const X_ICON: Asset = asset!("/assets/icons/x.svg");

// ReachMe section
#[component]
pub fn ReachMe() -> Element {
    rsx! {
        div {
            id: "reachme",
            class: "page-block page-section",
            div {
                class: "flex flex-col gap-12",

                h2 {
                    class: "block-title",
                    "reach me"
                }
                // Was `flex-direction: row; gap: 200px` unconditionally, which
                // overflowed narrow screens. Stacks below sm, side by side above.
                div {
                    class: "flex flex-col gap-12 sm:flex-row sm:gap-[clamp(3rem,12vw,12.5rem)]",
                    div {
                        class: "flex flex-col gap-5",
                        h3 {
                            class: "block-subtitle",
                            "my email"
                        }
                        // Was an <a> with no href: styled as a link, but not
                        // clickable and not keyboard focusable.
                        a {
                            class: "social-anchor text-lg break-all sm:text-2xl",
                            href: "mailto:denizhoroz.ofcl@gmail.com",
                            "denizhoroz.ofcl@gmail.com"
                        }
                    }
                    div {
                        class: "flex flex-col gap-5",
                        h3 {
                            class: "block-subtitle",
                            "my socials"
                        }
                        a {
                            class: "social-anchor button",
                            href: "https://github.com/denizhoroz",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            img { src: GITHUB_ICON, alt: "", class: "social-icon"}
                            "github"
                        }
                        a {
                            class: "social-anchor button",
                            href: "https://www.linkedin.com/in/denizhoroz",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            img { src: LINKEDIN_ICON, alt: "", class: "social-icon"}
                            "linkedin"
                        }
                        a {
                            class: "social-anchor button",
                            href: "https://x.com/denizerenhoroz",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            img { src: X_ICON, alt: "", class: "social-icon"}
                            "X"
                        }
                    }
                }
            }
        }
    }
}
