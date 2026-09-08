use dioxus::prelude::*;

use crate::{Route, ThemeToggle};

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
                    Link { to: Route::Home {}, active_class: "nav-active", "home" }
                    a { href: about_href, onclick: jump("aboutme"), "who am i?" }
                    Link { to: Route::MyWorksPage {}, active_class: "nav-active", "my works" }
                    a { href: reach_href, onclick: jump("reachme"), "reach me" }

                    // Inside the nav, not a third child of .navbar-inner --
                    // justify-between would pull the links off the right edge.
                    ThemeToggle {}
                }
            }
        }
        Outlet::<Route> {}
    }
}
