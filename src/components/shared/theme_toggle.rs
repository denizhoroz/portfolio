use dioxus::prelude::*;

// `data-theme` on <html> is the single source of truth: index.html sets it
// before first paint, this only flips it, and every colour hangs off
// :root[data-theme="dark"], so the swap is pure CSS with no re-render.
//
// Bare statements ending in `return`, not an IIFE -- Dioxus wraps the snippet in
// an async function and returns whatever it returns.
const TOGGLE_JS: &str = r#"
  const el = document.documentElement;
  const dark = el.dataset.theme !== "dark";
  el.dataset.theme = dark ? "dark" : "light";
  // A failed write must not stop the theme from changing for this session.
  try { localStorage.setItem("theme", dark ? "dark" : "light"); } catch (_) {}
  return dark;
"#;

// Read on mount: the button renders before WASM can see the DOM, so without this
// a dark-mode visitor briefly gets the wrong icon.
const READ_JS: &str = r#"return document.documentElement.dataset.theme === "dark";"#;

#[component]
pub fn ThemeToggle() -> Element {
    // Only the icon depends on this; the colours are already correct either way.
    let mut dark = use_signal(|| false);

    // eval().await already resolves to the script's return value.
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

    // stroke="currentColor" so the icon follows the theme.
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
            // The icon is aria-hidden, so without this the button announces as
            // just "button". States the action, not the state.
            aria_label: if dark() { "Switch to light theme" } else { "Switch to dark theme" },
            aria_pressed: "{dark()}",
            onclick: toggle,
            {icon}
        }
    }
}
