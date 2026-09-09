use dioxus::prelude::*;

use crate::{Footer, Route, ThemeToggle};

const TITLE_ICON: Asset = asset!("/assets/icons/titleicon.svg");

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

// Returns to the top of Home, for a click on "home" or on the brand mark.
//
// __HERE__ is whether Home was already the current route, and it decides both
// halves of this.
const SCROLL_TO_TOP_JS: &str = r#"
(() => {
  const here = __HERE__;
  const reduce = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

  // Smooth only when the reader is already on Home and can watch the page
  // travel. Arriving from another route the content is being replaced anyway,
  // so it lands at the top the way a real navigation would, without animating
  // over a page swap.
  window.scrollTo({ top: 0, left: 0, behavior: here && !reduce ? "smooth" : "auto" });

  // Only when already on Home: "/#aboutme" or "/#reachme" may still be in the
  // address bar from a nav jump, and the reader is no longer at that section.
  // Coming from another route the router has just written a clean "/" itself.
  // replaceState rather than push so the trip to the top is not a history entry
  // of its own, and history.state is kept so the router's entry survives.
  if (here) {
    history.replaceState(history.state, "", "/");
  }
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

    // Clicking "home" or the brand mark. One gesture, two jobs, and neither is
    // what a plain Link does on its own.
    //
    // Already on Home: the Link would ask the router to push the route it is
    // already displaying. The router correctly treats that as no navigation, so
    // nothing at all happens -- a reader sitting down in #reachme clicks "home"
    // and the page does not move. onclick_only below is what stops the pointless
    // push (it would otherwise stack a duplicate "/" entry on the back button),
    // which leaves the scrolling to this.
    //
    // Coming from another route: the router swaps the page but never touches the
    // viewport, and Home is four full-screen sections against a /works that is
    // one screen of cards. A reader who had scrolled down keeps that offset and
    // lands somewhere in the middle of Home rather than on the hero.
    //
    // Modifier and middle clicks never reach here -- Link returns before its
    // onclick for those -- so ctrl-click still opens Home in a new tab.
    let go_home = move |_evt: Event<MouseData>| {
        let js = SCROLL_TO_TOP_JS.replace("__HERE__", if on_home { "true" } else { "false" });
        spawn(async move {
            _ = document::eval(&js).await;
        });
    };

    rsx! {
        div {
            class: "navbar-container",

            // The bar is full-bleed for its background; this wrapper holds the
            // contents to the shared measure.
            div {
                class: "navbar-inner",

                // The mark is the whole brand now -- the "denizhoroz" wordmark
                // that used to sit beside it is gone, and the Link wraps the
                // image rather than standing next to it.
                Link {
                    class: "navbar-brand",
                    to: Route::Home {},
                    onclick: go_home,
                    // Suppresses the router push when Home is already showing,
                    // so clicking the mark on Home does not stack a duplicate
                    // "/" onto the back button. Off elsewhere, so the push still
                    // happens and go_home only resets the viewport after it.
                    onclick_only: on_home,

                    // alt must NOT be empty here, unlike the icons in the footer
                    // row. This image is the only thing inside the link, so its
                    // alt text is the link's accessible name -- blank it and a
                    // screen reader announces "link" with nothing after it.
                    img { src: TITLE_ICON, alt: "denizhoroz, home", class: "navbar-logo" }
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
                        // Same pairing as the brand mark above -- see go_home.
                        onclick: go_home,
                        onclick_only: on_home,
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
