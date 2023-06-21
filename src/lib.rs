use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Block {
    HEAD = 0,
    TAIL = 1,
    BODY = 2,
    FOOD = 3,
    WALL = 4,
    BLANK = 5,
}

pub enum Direction {
    TOP = 0,
    RIGHT = 1,
    BOTTOM = 2,
    LEFT = 3,
}

pub enum Status {
    PENDING = 0,
    RUNNING = 1,
    OVER = 2,
}

#[wasm_bindgen]
pub struct BigMap {
    width: usize,
    height: usize,
    blocks: Vec<Block>,
    snake: Vec<usize>,
    walls: Vec<usize>,
    food: usize,
    direction: Direction,
    status: Status,
}

impl BigMap {
    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get_pos(&self, index: usize) -> (usize, usize) {
        (index / self.width, index % self.width)
    }

    fn get_block(&self, x: usize, y: usize) -> Block {
        self.blocks[self.get_index(x, y)]
    }

    fn head_next_pos(&self, index: usize) -> (usize, usize) {
        let cur_pos = self.get_pos(index);
        let next_pos = match self.direction {
            Direction::TOP => (cur_pos.0, cur_pos.1 - 1),
            Direction::BOTTOM => (cur_pos.0, cur_pos.1 + 1),
            Direction::LEFT => (cur_pos.0 - 1, cur_pos.1),
            Direction::RIGHT => (cur_pos.0 + 1, cur_pos.1),
        };
        next_pos
    }

    fn failed_check(&self, next_index: usize) -> bool {
        let next_block = *self.blocks.get(next_index).unwrap();
        if next_block == Block::WALL || next_block == Block::BODY || next_block == Block::TAIL {
            true
        } else {
            false
        }
    }

    fn gen_food(&mut self) {
        let blank_blocks = self
            .blocks
            .iter()
            .enumerate()
            .filter(|(_, b)| **b == Block::BLANK);
        let mut rng = thread_rng();

        // Exclusive range
        let food_pos: usize = rng.gen_range(0..blank_blocks.count());
        self.food = food_pos;
    }

    fn update_snake(&mut self) {
        let last_value = self.snake.last().unwrap().clone();
        if last_value == self.food {
            self.snake.push(self.food);
        } else if self.failed_check(last_value) {
            self.status = Status::OVER;
        } else {
            let mut next_snake: Vec<usize> = Vec::new();
            for (index, part) in self.snake.iter().enumerate() {
                // 头
                if index == self.snake.len() - 1 {
                    let next_pos = self.head_next_pos(*part);
                    next_snake.push(self.get_index(next_pos.0, next_pos.1));
                } else {
                    next_snake.push(self.snake[index + 1]);
                }
            }
            self.snake = next_snake;
        }
    }

    fn render(&mut self) {
        let blocks: Vec<Block> = (0..self.width * self.height)
            .map(|i| {
                if self.walls.contains(&i) {
                    Block::WALL
                } else if self.snake.contains(&i) {
                    if i == self.snake[0] {
                        Block::TAIL
                    } else if i == self.snake[2] {
                        Block::HEAD
                    } else {
                        Block::BODY
                    }
                } else if i == self.food {
                    Block::FOOD
                } else {
                    Block::BLANK
                }
            })
            .collect();
        self.blocks = blocks;
    }

    pub fn start(&mut self) {
        self.render();
    }

    pub fn new(width: usize, height: usize, walls: Vec<(usize, usize)>) -> BigMap {
        let walls_pos: Vec<usize> = walls.iter().map(|w| w.1 * width + w.0).collect();

        let snake = vec![
            width * height / 2 - 1,
            width * height / 2,
            width * height / 2 + 1,
        ];

        let food = width * height / 2 + 4;

        let blocks: Vec<Block> = (0..width * height).map(|_index| Block::BLANK).collect();

        BigMap {
            width,
            height,
            blocks,
            snake,
            food,
            direction: Direction::RIGHT,
            status: Status::PENDING,
            walls: walls_pos,
        }
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}