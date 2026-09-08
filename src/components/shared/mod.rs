// Pieces that are not owned by any one route.
//
// Two kinds live here, and the distinction is worth keeping in mind when adding
// something:
//
//   page chrome     navbar and footer wrap every route (navbar is the router's
//                   #[layout]) and are a matched pair -- both are one
//                   --bar-height tall with the same rule across the page.
//   building blocks project_box and states are rendered by more than one route,
//                   so neither home/ nor works/ can own them.
//
// Anything used by exactly one route belongs in that route's folder instead.

// navbar.rs -- the routed layout, wraps every page
mod navbar;
pub use navbar::*;

// footer.rs -- the navbar's counterpart at the end of the page
mod footer;
pub use footer::*;

// theme_toggle.rs -- light/dark button, rendered by the navbar
mod theme_toggle;
pub use theme_toggle::*;

// project_box.rs -- the project card, used by home/my_works and works/list
mod project_box;
pub use project_box::*;

// states.rs -- skeleton and error UI for the async data seam, used by
// home/my_works, works/list and works/detail
mod states;
pub use states::*;
