// The "/" route.
//
// page.rs is the route component itself and does nothing but compose the four
// sections below it, so reordering the page or dropping a section in touches one
// file. Each section owns its own markup and its own assets.

// page.rs -- Home
mod page;
pub use page::*;

// hero.rs -- #hero
mod hero;
pub use hero::*;

// about_me.rs -- #aboutme. Lives here because it is a section of Home; if it is
// promoted to its own route it moves to an about/ folder of its own.
mod about_me;
pub use about_me::*;

// my_works.rs -- #myworks, the three-project preview
mod my_works;
pub use my_works::*;

// reach_me.rs -- #reachme, and the social icon assets
mod reach_me;
pub use reach_me::*;
