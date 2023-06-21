import { Game, Status, Block } from "hungry-snake";
import { memory } from "hungry-snake/hungry_snake_bg.wasm";

const width = 40;
const height = 50;

const wallsArr = [];
for (let i = 0; i < width; i++) {
  for (let j = 0; j < height; j++) {
    if (i === 0 || j === 0 || i === width - 1 || j === height - 1) {
      wallsArr.push(i + j * width);
    }
  }
}
const walls = new Uint32Array(wallsArr);

const game = Game.new(width, height, walls);

const canvas = document.getElementById("game") as HTMLCanvasElement;

const CELL_SIZE = 5; // px

canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const BLANK_COLOR = "#FFFFFF";
const WALL_COLOR = "#000000";
const FOOD_COLOR = "#fa2020";
const HEAD_COLOR = "#0a6feb";
const BODY_COLOR = "#2782f3";
const TAIL_COLOR = "#61a3f5";

const ctx = canvas.getContext("2d")!;

const drawGrid = () => {
  ctx.beginPath();
  ctx.lineWidth = 1 / window.devicePixelRatio;
  ctx.strokeStyle = BLANK_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};
const getIndex = (row: number, column: number) => {
  return row * width + column;
};
const drawCells = () => {
  const blockPtr = game.blocks();

  const cells = new Uint8Array(memory.buffer, blockPtr, width * height);
  const snakePtr = game.snake();
  const snake = new Uint32Array(memory.buffer, snakePtr, 4);
  console.log(snake);
  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      if (cells[idx] === Block.BLANK) {
        ctx.fillStyle = BLANK_COLOR;
      } else if (cells[idx] === Block.WALL) {
        ctx.fillStyle = WALL_COLOR;
      } else if (cells[idx] === Block.FOOD) {
        ctx.fillStyle = FOOD_COLOR;
      } else if (cells[idx] === Block.HEAD) {
        ctx.fillStyle = HEAD_COLOR;
      } else if (cells[idx] === Block.BODY) {
        ctx.fillStyle = BODY_COLOR;
      } else if (cells[idx] === Block.TAIL) {
        ctx.fillStyle = TAIL_COLOR;
      }

      ctx.fillRect(
        col * CELL_SIZE + 1,
        row * CELL_SIZE + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};
const renderLoop = () => {
  game.next_tick();

  drawGrid();
  drawCells();

//   requestAnimationFrame(renderLoop);
};
renderLoop();
