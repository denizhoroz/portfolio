use dioxus::prelude::*;

use crate::Route;

/// One step in the trail.
///
/// Two `Option`s, and they mean different things: `to` is `None` for the page
/// the reader is already on, and `label` is `None` while the name is still being
/// fetched. Build these with the constructors below rather than by hand -- they
/// are what keep the two from being confused at a call site.
#[derive(Clone, PartialEq)]
pub struct Crumb {
    /// `None` renders a placeholder of roughly title width.
    label: Option<String>,
    /// `None` marks the current page: rendered as text, never as a link.
    to: Option<Route>,
}

impl Crumb {
    /// An ancestor. Clickable.
    pub fn link(label: impl Into<String>, to: Route) -> Self {
        Self { label: Some(label.into()), to: Some(to) }
    }

    /// The page being viewed. Named, but not a link to itself.
    pub fn current(label: impl Into<String>) -> Self {
        Self { label: Some(label.into()), to: None }
    }

    /// The page being viewed, before its title has arrived.
    ///
    /// Only a detail route needs this: the last crumb there is the project or
    /// article title, which comes from the fetch rather than from the URL.
    pub fn pending() -> Self {
        Self { label: None, to: None }
    }
}

/// The path from the site root to the current page, every ancestor clickable.
///
/// Replaces the single "go back" link that used to sit in this slot. That link
/// only ever named one ancestor, and it described a hierarchy it could not show:
/// a reader landing on /works/interloper from a shared link had no way to tell
/// that /works existed, and no way to reach it except the navbar. A trail states
/// the whole ancestry and makes every level of it a target.
///
/// Rendered on every route but Home. Home is the root, so its trail would be a
/// single non-clickable crumb naming the page the reader is looking at.
#[component]
pub fn Breadcrumb(trail: Vec<Crumb>) -> Element {
    rsx! {
        // nav + ol is the shape assistive tech expects for a breadcrumb: the
        // landmark names the region, the list gives it a length and a position.
        // aria_label is what distinguishes this landmark from the navbar's.
        nav {
            class: "breadcrumb",
            aria_label: "Breadcrumb",

            ol {
                class: "breadcrumb-list",

                for (i, crumb) in trail.iter().enumerate() {
                    li {
                        key: "{i}",
                        class: "breadcrumb-item",

                        // Inside the <li> rather than between them, so the
                        // separator cannot be read as a list item of its own.
                        // aria_hidden because "/" is punctuation here; screen
                        // readers get the structure from the list.
                        if i > 0 {
                            span { class: "breadcrumb-sep", aria_hidden: "true", "/" }
                        }

                        {match (&crumb.to, &crumb.label) {
                            (Some(route), Some(label)) => rsx! {
                                Link {
                                    class: "breadcrumb-link",
                                    to: route.clone(),
                                    "{label}"
                                }
                            },

                            (None, Some(label)) => rsx! {
                                span {
                                    class: "breadcrumb-current",
                                    aria_current: "page",
                                    "{label}"
                                }
                            },

                            // Still loading. A sized placeholder rather than an
                            // empty span: the crumb has to hold its line so the
                            // title arriving does not reflow the trail.
                            (_, None) => rsx! {
                                span {
                                    class: "skeleton breadcrumb-pending",
                                    aria_hidden: "true"
                                }
                            },
                        }}
                    }
                }
            }
        }
    }
}
