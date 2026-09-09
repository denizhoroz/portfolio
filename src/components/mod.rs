// One folder per route, plus shared/ for what outlives any single route.
// A section lives with the route that renders it; promoting one to its own route
// is a folder move, because everything is reached through `crate::Name` (main.rs
// does `use components::*`) rather than through its path.

mod home;
pub use home::*;

mod works;
pub use works::*;

mod articles;
pub use articles::*;

mod shared;
pub use shared::*;
