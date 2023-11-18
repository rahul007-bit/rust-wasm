import init, { World, Direction, Status } from "rust-wasm";
import { rnd } from "./utils/rnd";

const stats = document.getElementById("stats");
const changeStatus = document.getElementById("change-status");

init().then((wasm) => {
  const CELL_SIZE = 20;
  const WORLD_W = 8;
  const SNAKE_INDEX = rnd(WORLD_W * WORLD_W);
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
    const snakeCellPointer = new Uint32Array(
      memory.buffer,
      snakeCell,
      snakeLength
    );
    snakeCellPointer
      .filter((cellIdx) => !(cellIdx !== 0 && cellIdx === snakeCellPointer[0]))
      .forEach((cellIdx, i) => {
        const col = cellIdx % worldWidth;
        const row = Math.floor(cellIdx / worldWidth);
        ctx.fillStyle = i === 0 ? "green" : "black";
        ctx.beginPath();
        ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
      });
    ctx.stroke();
  }

  const printWorld = () => {
    drawWorld();
    drawSnake();
    drawReward();
  };

  const drawReward = () => {
    const rewardCell = world.reward_cell();
    const col = rewardCell % worldWidth;
    const row = Math.floor(rewardCell / worldWidth);
    ctx.fillStyle = "red";
    ctx.beginPath();
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    ctx.stroke();
    const status = world.status();
    switch (status) {
      case Status.Win:
        stats.innerText = "You Win";
        changeStatus.innerText = "Restart";
        break;
      case Status.Dead:
        stats.innerText = "Game Over";
        changeStatus.innerText = "Restart";
        break;
      default:
        break;
    }
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

  changeStatus.addEventListener("click", () => {
    world.set_status();
    const status = world.status();
    switch (status) {
      case Status.Alive:
        changeStatus.innerText = "Pause";
        stats.innerText = "Playing";
        break;
      case Status.Dead:
        changeStatus.innerText = "Restart";
        stats.innerText = "Game Over";
        break;
      case Status.Pause:
        changeStatus.innerText = "Resume";
        stats.innerText = "Paused";
        break;
    }
  });

  updateWorld();
});
