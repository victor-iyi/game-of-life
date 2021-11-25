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
use std::fmt;

use wasm_bindgen::prelude::*;

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Rendering options for the Game of life.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderOptions {
  /// Render using `<canvas>` element.
  ///
  /// Uses the HTML5 canvas API.
  Canvas,
  /// Render using `<pre>` element.
  ///
  /// Renders the entire universe as a single string.
  /// Use when on a low memory device.
  Text,
}

/// Each cell in the universe is represented as a single-byte.
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
  /// Dead cell.
  Dead = 0,
  /// Alive cell.
  Alive = 1,
}

/// The Game of Life universe.
#[wasm_bindgen]
pub struct Universe {
  /// The width of the universe.
  width: u32,
  /// The height of the universe.
  height: u32,
  /// The cells of the universe of length `width * height`.
  cells: Vec<Cell>,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
  /// Create a new universe with the given width and height.
  pub fn new(width: u32, height: u32) -> Universe {
    utils::set_panic_hook();

    let cells = (0..width * height)
      .map(|i| {
        // if js_sys::Math::random() < 0.5 {
        if i % 2 == 0 || i % 7 == 0 {
          Cell::Alive
        } else {
          Cell::Dead
        }
      })
      .collect();

    Universe {
      width,
      height,
      cells,
    }
  }

  /// Encode the rules of the universe to determine
  /// if the neighbor cell is alive or dead.
  ///
  /// The rules are:
  ///
  /// - **Rule 1**: Any live cell with fewer than two live neighbours dies,
  ///               as if caused by underpopulation.
  /// - **Rule 2**: Any live cell with two or three live neighbours lives
  ///               on to the next generation.
  /// - **Rule 3**: Any live cell with more than three live neighbours dies,
  ///               as if by overpopulation.
  /// - **Rule 4**: Any dead cell with exactly three live neighbours becomes a live cell,
  ///               as if by reproduction.
  ///  - **Otherwise**: All other cells remain in the same state.
  ///
  pub fn tick(&mut self) {
    let mut next = self.cells.clone();

    for row in 0..self.height {
      for col in 0..self.width {
        let idx = self.get_index(row, col);
        let cell = self.cells[idx];
        let live_neighbors = self.live_neighbor_count(row, col);

        let next_cell = match (cell, live_neighbors) {
          // Rule 1: Any live cell with fewer than two live neighbours
          // dies, as if caused by underpopulation.
          (Cell::Alive, x) if x < 2 => Cell::Dead,
          // Rule 2: Any live cell with two or three live neighbours
          // lives on to the next generation.
          (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
          // Rule 3: Any live cell with more than three live
          // neighbours dies, as if by overpopulation.
          (Cell::Alive, x) if x > 3 => Cell::Dead,
          // Rule 4: Any dead cell with exactly three live neighbours
          // becomes a live cell, as if by reproduction.
          (Cell::Dead, 3) => Cell::Alive,
          // All other cells remain in the same state.
          (otherwise, _) => otherwise,
        };
        next[idx] = next_cell;
      }
    }
    self.cells = next;
  }

  /// Render the universe as a string.
  ///
  /// To enable this behavior, set the `RENDER_OPTIONS`
  /// environment variable to `RenderOptions::Text`.
  pub fn render(&self) -> String {
    self.to_string()
  }

  /// Get the width of the universe.
  pub fn width(&self) -> u32 {
    self.width
  }

  /// Get the height of the universe.
  pub fn height(&self) -> u32 {
    self.height
  }

  /// Get the entire cells in the universe.
  pub fn cells(&self) -> *const Cell {
    self.cells.as_ptr()
  }
}

/// Private methods.
impl Universe {
  /// The row and column are translated into an index into the
  /// cells vector to access the cell at a given row and column.
  fn get_index(&self, row: u32, column: u32) -> usize {
    (row * self.width + column) as usize
  }

  /// Get the state of a cell at a given row and column.
  ///
  /// Get the count of how many neighbors are alive,
  /// to estimate the next state of the cell.
  fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
    let mut count = 0;
    for delta_row in [self.height - 1, 0, 1].iter().cloned() {
      for delta_col in [self.width - 1, 0, 1].iter().cloned() {
        if delta_row == 0 && delta_col == 0 {
          continue;
        }

        let neighbor_row = (row + delta_row) % self.height;
        let neighbor_col = (column + delta_col) % self.width;
        let idx = self.get_index(neighbor_row, neighbor_col);
        count += self.cells[idx] as u8;
      }
    }
    count
  }
}

impl fmt::Display for Universe {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for line in self.cells.as_slice().chunks(self.width as usize) {
      for &cell in line {
        let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
        write!(f, "{}", symbol)?;
      }
      write!(f, "\n")?;
    }

    Ok(())
  }
}
