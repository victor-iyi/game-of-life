//! # Game of Life
//!
//! The Game of Life is played in an infinite universe, but we don't have inifinite
//! memory and compute power. Working around this rather annoying limitation usually
//! comes in one of three flavors:
//!
//! 1. Keep track of which subset of the universe has interesting things happening,
//! and expand this region as needed. In the wrost case, this expansion is unbounded
//! and the implementation will get slower eventually run out of memory.
//!
//! 2. Create a fixed-size universe, where cells on the edges have fewer neighbors
//! than cells in the middle. The downside with this approach is that infinite patterns,
//! like gliders, that reach the end of the universe are snuffed out.
//!
//! 3. Create a fixed-size, periodic universe, where cells on the edges have neighbors
//! that wrap around to the other side of the universe. Because neighbors wrap around
//! the edges of the universe, gliders can keep running forever.
//!
//! The third option is implemented here.

#[macro_use]
mod macros;

mod cells;
mod options;
mod universe;
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub use cells::Cell;
pub use options::RenderOptions;
pub use universe::Universe;
