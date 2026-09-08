use dioxus::prelude::*;

// Flips the theme and remembers it.
//
// The `data-theme` attribute is the single source of truth -- index.html sets it
// synchronously before the first paint, and this only ever flips it. Nothing
// re-renders the page on a theme change: every colour is a CSS custom property
// hanging off :root[data-theme="dark"], so the swap is pure CSS.
//
// Returns the theme it just moved to, so the signal below and the DOM cannot
// drift apart.
// Bare statements ending in `return`, not an IIFE: Dioxus wraps the snippet in
// an async function and hands back whatever it returns, so an IIFE expression
// statement would evaluate and then throw the value away.
const TOGGLE_JS: &str = r#"
  const el = document.documentElement;
  const dark = el.dataset.theme !== "dark";
  el.dataset.theme = dark ? "dark" : "light";
  // A failed write must not stop the theme from changing for this session.
  try { localStorage.setItem("theme", dark ? "dark" : "light"); } catch (_) {}
  return dark;
"#;

// Read on mount. The button renders before WASM can see the DOM, so without this
// a dark-mode visitor would briefly get the wrong icon on an otherwise correct
// page.
const READ_JS: &str = r#"return document.documentElement.dataset.theme === "dark";"#;

#[component]
pub fn ThemeToggle() -> Element {
    // Starts light and is corrected on mount -- see READ_JS. Only the icon
    // depends on this; the colours are already right either way.
    let mut dark = use_signal(|| false);

    // `eval(..).await` already resolves to the script's return value, as a
    // serde_json Value -- there is no separate recv step.
    use_effect(move || {
        spawn(async move {
            if let Ok(v) = document::eval(READ_JS).await {
                dark.set(v.as_bool().unwrap_or(false));
            }
        });
    });

    let toggle = move |_| {
        spawn(async move {
            if let Ok(v) = document::eval(TOGGLE_JS).await {
                dark.set(v.as_bool().unwrap_or(false));
            }
        });
    };

    // stroke="currentColor" on both, so the icon follows --color-ink through the
    // theme swap instead of carrying its own hardcoded colour.
    let icon = if dark() {
        // Sun: clicking goes back to light.
        rsx! {
            svg {
                view_box: "0 0 24 24", fill: "none", stroke: "currentColor",
                stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                "aria-hidden": "true", class: "theme-icon",
                circle { cx: "12", cy: "12", r: "4" }
                path { d: "M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M6.34 17.66l-1.41 1.41M19.07 4.93l-1.41 1.41" }
            }
        }
    } else {
        // Moon: clicking goes to dark.
        rsx! {
            svg {
                view_box: "0 0 24 24", fill: "none", stroke: "currentColor",
                stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                "aria-hidden": "true", class: "theme-icon",
                path { d: "M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" }
            }
        }
    };

    rsx! {
        button {
            r#type: "button",
            class: "theme-toggle",
            // The icon is aria-hidden, so without this the button is announced
            // as just "button". The label states the action, not the state.
            aria_label: if dark() { "Switch to light theme" } else { "Switch to dark theme" },
            // Lets a screen reader announce the current mode as well as the action.
            aria_pressed: "{dark()}",
            onclick: toggle,
            {icon}
        }
    }
}
