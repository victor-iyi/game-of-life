import { Universe, Cell, RenderOptions } from "game-of-life";
import { memory } from "game-of-life/game_of_life_bg.wasm";


// Rendering options:
// -- Canvas (recommended)
// -- Text (set when using low memory device)
const RENDER_OPTIONS = RenderOptions.Canvas;

// Universe represents the universe of cells.
const universe = Universe.new(64, 64);
const width = universe.width();
const height = universe.height();

// Render loop for the game of life.
let renderLoop;

if (RENDER_OPTIONS === RenderOptions.Text) {
  const pre = document.getElementById("game-of-life-text");
  renderLoop = () => {
    pre.textContent = universe.render();
    universe.tick();
    requestAnimationFrame(renderLoop);
  };
} else if (RENDER_OPTIONS === RenderOptions.Canvas) {
  const CELL_SIZE = 5; // px
  const GRID_COLOR = "#ccc";
  const DEAD_COLOR = "#fff";
  const ALIVE_COLOR = "#000";

  // Give the canvas room for all of our cells and a 1px border
  // around each of them.
  const canvas = document.getElementById("game-of-life-canvas");
  canvas.width = (CELL_SIZE + 1) * width + 1;
  canvas.height = (CELL_SIZE + 1) * height + 1;

  const ctx = canvas.getContext("2d");

  renderLoop = () => {
    universe.tick();

    drawGrid();
    drawCells();

    requestAnimationFrame(renderLoop);
  };

  const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
      ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
      ctx.lineTo(i * (CELL_SIZE + 1) + 1, (height + 1) * (CELL_SIZE + 1));
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
      ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
      ctx.lineTo(width * (CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
  };

  const getIndex = (row, column) => {
    return row * width + column;
  };

  const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        const idx = getIndex(row, col);

        ctx.fillStyle = cells[idx] === Cell.Dead
          ? DEAD_COLOR
          : ALIVE_COLOR;

        ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        );
      }
    }

    ctx.stroke();
  };

  drawGrid();
  drawCells();
  requestAnimationFrame(renderLoop);
} else {
  console.error("Unknown render options");
}

requestAnimationFrame(renderLoop);
