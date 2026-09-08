// Every component is re-exported at this level, and main.rs does
// `use components::*`, which is what lets a sibling module reach another one
// through `crate::` (see the `use crate::{...}` lines in works.rs, my_works.rs
// and navbar.rs).

// ---- pages ----

// home.rs -- composes the four Home sections below
mod home;
pub use home::*;

// works.rs
mod works;
pub use works::*;

// work.rs
mod work;
pub use work::*;

// ---- Home sections, one file each ----

// hero.rs
mod hero;
pub use hero::*;

// about_me.rs
mod about_me;
pub use about_me::*;

// my_works.rs
mod my_works;
pub use my_works::*;

// reach_me.rs
mod reach_me;
pub use reach_me::*;

// footer.rs
mod footer;
pub use footer::*;

// ---- shared ----

// navbar.rs -- the routed layout wrapping every page
mod navbar;
pub use navbar::*;

// project_box.rs -- the card, shared by MyWorks and /works
mod project_box;
pub use project_box::*;

// states.rs -- loading / error UI for the async data seam
mod states;
pub use states::*;
