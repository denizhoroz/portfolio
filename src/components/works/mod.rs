// The "/works" and "/works/:slug" routes.
//
// Both live here because the second is a child of the first -- the detail page
// is only ever reached from the list.
//
// Named list/detail rather than works/work: the old pair of filenames differed
// by a single character and was genuinely easy to misread.

// list.rs -- MyWorksPage, "/works"
mod list;
pub use list::*;

// detail.rs -- WorkPage, "/works/:slug"
mod detail;
pub use detail::*;
