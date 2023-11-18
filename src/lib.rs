use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/www/utils/rnd.js")]
extern "C" {
    fn rnd(max: usize) -> usize;
}

#[derive(Clone, PartialEq)]
#[wasm_bindgen]
pub struct SnakeCell(usize);

#[derive(Clone, Copy,PartialEq)]
#[wasm_bindgen]
pub enum Status {
    Alive,
    Dead,
    Win,
    Pause,
}

#[wasm_bindgen]
struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

#[derive(Clone, Copy, PartialEq)]
#[wasm_bindgen]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Snake {
        let mut body = vec![];

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }

        Snake {
            body,
            direction: Direction::Down,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    reward_cell: usize,
    status: Status,
    reward: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> World {
        let size = width * width;
        let snake = Snake::new(snake_idx, 3);

        let body_as_usize: Vec<usize> = snake.body.iter().map(|cell| cell.0).collect();

        let new_world = World {
            width,
            size,
            reward_cell: World::gen_reward_cell(size, &body_as_usize),
            snake,
            status: Status::Pause,
            reward: 0,
        };

        new_world
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn set_status(&mut self) {
        if self.status == Status::Pause {
            self.status = Status::Alive;
        } else if self.status == Status::Alive {
            self.status = Status::Pause;
        } else if self.status == Status::Dead {
            self.restart();
        } else if self.status == Status::Win {
            self.restart();
        }
    }

    pub fn restart(&mut self) {
        self.snake = Snake::new(self.size / 2, 3);
        let body_as_usize: Vec<usize> =
            self.snake.body.iter().map(|cell| cell.0).collect();
        self.reward_cell = World::gen_reward_cell(self.size, &body_as_usize);
        self.status = Status::Pause;
        self.reward = 0;
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn score(&self) -> usize {
        self.reward
    }

    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn snake_len(&self) -> usize {
        self.snake.body.len()
    }

    pub fn change_direction(&mut self, direction: Direction) {
        let next_cell = self.gen_next_snake_cell(direction);
        if self.snake.body[1].0 == next_cell.0 {
            return;
        }
        self.snake.direction = direction;
    }

    pub fn reward_cell(&self) -> usize {
        self.reward_cell
    }

    pub fn gen_reward_cell(max: usize, body: &[usize]) -> usize {
        let mut reward_cell;
        loop {
            reward_cell = rnd(max);
            if !body.contains(&reward_cell) {
                break;
            }
        }
        reward_cell
    }

    pub fn step(&mut self) {
        match self.status {
            Status::Alive => {
                let temp = self.snake.body.clone();
                let next_cell = self.gen_next_snake_cell(self.snake.direction);
                self.snake.body[0] = next_cell;
                let len = self.snake.body.len();

                for i in 1..len {
                    self.snake.body[i] = SnakeCell(temp[i - 1].0);
                }

                if self.snake.body[1..].contains(&self.snake.body[0]) {
                    self.status = Status::Dead;
                }

                if self.reward_cell == self.snake_head_idx() {
                    if self.snake.body.len() < self.size {
                        let body_as_usize: Vec<usize> =
                            self.snake.body.iter().map(|cell| cell.0).collect();
                        self.reward_cell = World::gen_reward_cell(self.size, &body_as_usize);
                        self.reward += 1;
                    } else {
                        self.reward_cell = self.size + 1;
                        self.status = Status::Win;
                    }
                    self.snake.body.push(SnakeCell(temp[len - 1].0));
                }
            }
            Status::Dead => {
                self.status = Status::Dead;
            }
            Status::Win => {
                self.status = Status::Win;
            }
            Status::Pause => {
                self.status = Status::Pause;
            }
        }
    }
    pub fn gen_next_snake_cell(&self, direction: Direction) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;

        match direction {
            Direction::Right => {
                let threshold = (row + 1) * self.width;
                if snake_idx + 1 == threshold {
                    SnakeCell(threshold - self.width)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            }
            Direction::Left => {
                let threshold = row * self.width;
                if snake_idx == threshold {
                    SnakeCell(threshold + (self.width - 1))
                } else {
                    SnakeCell(snake_idx - 1)
                }
            }
            Direction::Up => {
                let threshold = snake_idx - (row * self.width);
                if snake_idx == threshold {
                    SnakeCell((self.size - self.width) + threshold)
                } else {
                    SnakeCell(snake_idx - self.width)
                }
            }
            Direction::Down => {
                let threshold = snake_idx + ((self.width - row) * self.width);
                if snake_idx + self.width == threshold {
                    SnakeCell(threshold - ((row + 1) * self.width))
                } else {
                    SnakeCell(snake_idx + self.width)
                }
            }
        }
    }
}

// Direction::Right=>{
//     let threshold = (row + 1) * self.width;
//     if snake_idx + 1 >= threshold {
//         SnakeCell(threshold - self.width)
//     } else {
//         SnakeCell(snake_idx + 1)
//     }
// }
// Direction::Left=>{
//     let threshold = row * self.width;
//     if snake_idx == threshold {
//         SnakeCell(threshold +( self.width - 1))
//     } else {
//         SnakeCell(snake_idx - 1)
//     }
//     // SnakeCell((row * self.width) + ((snake_idx - 1) % self.width))
// }
// Direction::Up=>{
//     let threshold = snake_idx - (row-self.width);
//     if snake_idx - self.width < threshold {
//         SnakeCell(snake_idx - self.width + threshold)
//     } else {
//         SnakeCell(snake_idx - self.width)
//     }
//     // SnakeCell((snake_idx - self.width) % self.size)
// }
// Direction::Down=>{
//     let threshold = snake_idx + ((self.width - row)*self.width);
//     if snake_idx + self.width == threshold {
//         SnakeCell(threshold - ((row-1)*self.width    ))
//     } else {
//         SnakeCell(snake_idx + self.width)
//     }
// }
