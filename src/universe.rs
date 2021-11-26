// Copyright 2021 Victor I. Afolabi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt;

use wasm_bindgen::prelude::*;

use crate::{cells::Cell, utils};

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

        log!(
          "cell[{}, {}] is initially {:?} and has {} live neighbors",
          row,
          col,
          cell,
          live_neighbors
        );

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

  /// Set the width of the universe.
  ///
  /// Resets all cells to dead state.
  pub fn set_width(&mut self, width: u32) {
    self.width = width;
    self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
  }

  /// Get the height of the universe.
  pub fn height(&self) -> u32 {
    self.height
  }

  /// Set the height of the universe.
  ///
  /// Resets all cells to dead state.
  pub fn set_height(&mut self, height: u32) {
    self.height = height;
    self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
  }

  /// Get the entire cells in the universe.
  pub fn cells(&self) -> *const Cell {
    self.cells.as_ptr()
  }
}

impl Universe {
  /// Get the dead and alive cells in the entire universe.
  pub fn get_cells(&self) -> &[Cell] {
    &self.cells
  }

  /// Set cells to be alive in a universe by passing the row and column
  /// of each cell as an array.
  ///
  /// Example:
  ///
  /// ```rust
  /// use game_of_life::Universe;
  /// let mut universe = Universe::new(5, 5);
  /// universe.set_cells(&[(1, 2), (2, 3), (3, 4), (4, 5)]);
  /// ```
  pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
    for (row, col) in cells.iter().cloned() {
      let idx = self.get_index(row, col);
      self.cells[idx] = Cell::Alive;
    }
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
