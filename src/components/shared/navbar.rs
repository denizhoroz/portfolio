use dioxus::prelude::*;

use crate::{Footer, Route, ThemeToggle};

// Scrolls to an in-page section once the router has actually rendered it. After
// a route push Home mounts on a later frame, so the target does not exist at
// click time -- poll for it, then write the hash so the URL stays shareable.
const SCROLL_TO_SECTION_JS: &str = r#"
(() => {
  const id = "__ID__";
  const reduce = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  let tries = 60;
  const go = () => {
    const el = document.getElementById(id);
    if (el) {
      // "center", not "start": sections are a full screen tall, so "start"
      // pins the heading against the navbar with the section below it.
      el.scrollIntoView({ behavior: reduce ? "auto" : "smooth", block: "center" });
      // Keep whatever state the router put on this entry; only the URL changes.
      history.replaceState(history.state, "", "/#" + id);
    } else if (tries-- > 0) {
      requestAnimationFrame(go);
    }
  };
  requestAnimationFrame(go);
})();
"#;

#[component]
pub fn Navbar() -> Element {
    // A bare "#aboutme" scrolls in place. "/#aboutme" is a same-origin document
    // navigation, which in a WASM app re-downloads and re-initialises the whole
    // binary -- so it is only used when we are genuinely on another route.
    let on_home = matches!(use_route::<Route>(), Route::Home {});
    let about_href = if on_home { "#aboutme" } else { "/#aboutme" };
    let reach_href = if on_home { "#reachme" } else { "/#reachme" };

    // Breathing room at the top and bottom of every route, applied here rather
    // than in each page: this is the one place every route passes through, so a
    // page added later gets it without remembering to.
    //
    // Home is the exception. Its children are full-height .page-section blocks
    // measured against the viewport, so padding around them makes the last
    // section overflow by exactly the padding and puts a scrollbar on a page
    // designed to have none.
    let outlet_class = if on_home { "" } else { "page-pad" };

    // The href stays a real URL for the status bar and open-in-new-tab, but the
    // plain click is intercepted: letting the browser follow it re-downloads the
    // WASM binary AND looks for the target before Dioxus has rendered it.
    let jump = move |id: &'static str| {
        move |evt: Event<MouseData>| {
            if on_home {
                // Same-page anchor: the browser already does the right thing.
                return;
            }
            evt.prevent_default();
            navigator().push(Route::Home {});
            let js = SCROLL_TO_SECTION_JS.replace("__ID__", id);
            spawn(async move {
                _ = document::eval(&js).await;
            });
        }
    };

    rsx! {
        div {
            class: "navbar-container",

            // The bar is full-bleed for its background; this wrapper holds the
            // contents to the shared measure.
            div {
                class: "navbar-inner",

                div {
                    class: "navbar-title",
                    Link { to: Route::Home {}, "denizhoroz" }
                }

                nav {
                    id: "navbar",
                    // active_class also sets aria-current="page" for screen readers
                    // Not active_class, which the other three keep: it matches
                    // on the whole URL, and "who am i?" / "reach me" write
                    // "/#aboutme" and "/#reachme" into it. That stops matching
                    // "/" and unbolds home while the reader is still on it.
                    // on_home is the question actually being asked, and it
                    // ignores the hash.
                    Link {
                        to: Route::Home {},
                        class: if on_home { "nav-active" } else { "" },
                        aria_current: if on_home { "page" } else { "false" },
                        "home"
                    }
                    a { href: about_href, onclick: jump("aboutme"), "who am i?" }
                    Link { to: Route::WorksPage {}, active_class: "nav-active", "works" }
                    Link { to: Route::ArticlesPage {}, active_class: "nav-active", "articles" }
                    a { href: reach_href, onclick: jump("reachme"), "reach me" }

                    // Inside the nav, not a third child of .navbar-inner --
                    // justify-between would pull the links off the right edge.
                    ThemeToggle {}
                }
            }
        }
        // One column, at least one screen tall, holding the route and the
        // footer. The height is what stops a short page -- /articles with two
        // rows, a project page with no body -- ending partway up the viewport
        // with the page background showing beneath the footer.
        div {
            class: "page-shell",

            div {
                class: outlet_class,
                Outlet::<Route> {}
            }

            // Every route ends here. Rendered by the layout rather than by each
            // page so a route added later cannot ship without one, and so the
            // spacing above it is defined in a single place.
            //
            // Deliberately in normal flow, not fixed like the navbar: this
            // marks the end of the content, and a bar pinned to the bottom of
            // the viewport would claim that the page ends wherever the reader
            // happens to have stopped scrolling.
            //
            // mt-auto takes whatever height the route above did not use, so the
            // footer sits on the bottom edge of a short page and immediately
            // after the content of a long one.
            div {
                class: "mt-auto pt-[clamp(4rem,12vw,10rem)]",
                Footer {}
            }
        }
    }
}
