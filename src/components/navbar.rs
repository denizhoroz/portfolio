use dioxus::prelude::*;

use crate::{Route, ThemeToggle};

// Shared navbar component.
// Scrolls to an in-page section once the router has actually rendered it.
// After a route push Home mounts on a later frame, so the target element does
// not exist at click time -- poll a bounded number of frames for it, then write
// the hash into the URL so the location stays shareable and copyable.
const SCROLL_TO_SECTION_JS: &str = r#"
(() => {
  const id = "__ID__";
  const reduce = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  let tries = 60;
  const go = () => {
    const el = document.getElementById(id);
    if (el) {
      // "center" rather than "start": the sections are a full screen tall, so a
      // "start" landing puts the heading hard against the navbar with the whole
      // section below it. (This used to also match the snap controller's
      // alignment; that is gone, but centring is still the right landing.)
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

    // Off Home the href stays a real "/#section" URL -- it is what the status
    // bar shows and what open-in-new-tab uses -- but the plain click is
    // intercepted. Letting the browser follow it did two wrong things at once:
    // it re-downloaded the whole WASM binary, and it looked for #reachme before
    // Dioxus had rendered anything, so the scroll silently failed and the user
    // was dropped at the top of Home.
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

            // The bar is full-bleed so its background can occlude content
            // scrolling underneath; this inner wrapper is what holds the title
            // and links to the same measure and gutter as .page-block, so they
            // sit exactly above the content edges below.
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

                    // Inside the nav rather than a third child of .navbar-inner:
                    // that wrapper is justify-between, so a third child would
                    // pull the links away from the right edge they are aligned
                    // to. Here the toggle simply becomes the last item.
                    ThemeToggle {}
                }
            }
        }
        Outlet::<Route> {}
    }
}
