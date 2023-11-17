import init, { World, Direction,} from "rust-wasm";

init().then((wasm) => {

const CELL_SIZE = 20;
const WORLD_W = 8;
const SNAKE_INDEX = Date.now() % (WORLD_W * WORLD_W);
const FPS = 4;


const memory = wasm.memory;

const world = World.new(WORLD_W, SNAKE_INDEX);
const worldWidth = world.width();


const canvas = <HTMLCanvasElement>document.getElementById("snake-canvas");
const ctx = canvas.getContext("2d");

canvas.height = worldWidth * CELL_SIZE;
canvas.width = worldWidth * CELL_SIZE;

function drawWorld() {
  ctx.beginPath();

  for (let x = 0; x < worldWidth + 1; x++) {
    ctx.moveTo(CELL_SIZE * x, 0);
    ctx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE);
  }

  for (let y = 0; y < worldWidth + 1; y++) {
    ctx.moveTo(0, CELL_SIZE * y);
    ctx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y);
  }

  ctx.stroke();
}

function drawSnake() {
  const snakeCell = world.snake_cells();
  const snakeLength = world.snake_len();
  const snakeCellPointer = new Uint32Array(memory.buffer, snakeCell, snakeLength);
  snakeCellPointer.forEach((cellIdx,i) =>{

    const col = cellIdx % worldWidth;
    const row = Math.floor(cellIdx / worldWidth);
    ctx.fillStyle = i === 0 ? "green" : "black";
    ctx.beginPath();
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);}
  );
  ctx.stroke();
}

const printWorld = () => {
  drawWorld();
  drawSnake();
};

const updateWorld = () => {
  setTimeout(() => {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    world.step();
    printWorld();
    requestAnimationFrame(updateWorld);
  }, 1000 / FPS);
};

// listen for key-presses

document.addEventListener("keydown", (e) => {
  switch (e.key) {
    case "ArrowUp":
      world.change_direction(Direction.Up);
      break;
    case "ArrowRight":
      world.change_direction(Direction.Right);
      break;
    case "ArrowDown":
      world.change_direction(Direction.Down);
      break;
    case "ArrowLeft":
      world.change_direction(Direction.Left);
      break;
  }
});

updateWorld();

});
