// One folder per route, plus one for the pieces that outlive any single route.
//
//   home/    the "/" route and the sections it is built from
//   works/   the "/works" list and the "/works/:slug" detail page
//   shared/  chrome and building blocks used by more than one route
//
// A section lives with the route that renders it -- about_me.rs sits in home/
// because "who am i?" is currently a section of the Home page. If it becomes its
// own route later, the file moves to an about/ folder and nothing else changes:
// every component is reached through `crate::Name`, not through its path.
//
// That indirection is what the flat re-exports below buy. main.rs does
// `use components::*`, so `crate::ProjectBox` resolves regardless of which
// folder ProjectBox actually lives in -- which is why this reshuffle needed no
// edits inside the components themselves.

mod home;
pub use home::*;

mod works;
pub use works::*;

mod shared;
pub use shared::*;
