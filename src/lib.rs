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

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Status {
    PENDING = 0,
    RUNNING = 1,
    OVER = 2,
}

#[wasm_bindgen]
pub struct Game {
    width: usize,
    height: usize,
    blocks: Vec<Block>,
    snake: Vec<usize>,
    walls: Vec<usize>,
    food: Option<usize>,
    direction: Direction,
    status: Status,
}

#[wasm_bindgen]
impl Game {
    fn set_dir(&mut self, next_direction: Direction) {
        self.direction = next_direction;
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get_pos(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
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
        let mut blank_blocks = Vec::new();
        for (index, b) in self.blocks.iter().enumerate() {
            if *b == Block::BLANK {
                blank_blocks.push(index);
            }
        }
        let mut rng = thread_rng();

        let food_pos: usize = rng.gen_range(0..blank_blocks.len());
        let food_index = blank_blocks[food_pos];
        self.food = Some(food_index);
    }

    fn clear_food(&mut self) {
        self.food = None;
    }

    fn update_snake(&mut self) {
        let last_value = self.snake.last().unwrap().clone();
        let food_pos = self.food.unwrap();
        if last_value == food_pos {
            self.snake.push(food_pos);
            self.clear_food();
        } else if self.failed_check(last_value) {
            self.status = Status::OVER;
        } else {
            let mut next_snake: Vec<usize> = Vec::new();
            let next_head_pos = self.head_next_pos(self.snake.last().unwrap().clone());
            let next_head = self.get_index(next_head_pos.0, next_head_pos.1);
            if next_head == self.food.unwrap() {
                self.snake.push(next_head);
                self.gen_food();
            } else {
                for (index, part) in self.snake.iter().enumerate() {
                    // 头
                    if index == self.snake.len() - 1 {
                        let next_pos = self.head_next_pos(*part);
                        next_snake.push(self.get_index(next_pos.0, next_pos.1));
                    } else {
                        next_snake.push(self.snake.get(index + 1).unwrap().clone());
                    }
                }
                self.snake = next_snake;
            }
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
                    } else if i == self.snake[self.snake.len() - 1] {
                        Block::HEAD
                    } else {
                        Block::BODY
                    }
                } else if i == self.food.unwrap() {
                    Block::FOOD
                } else {
                    Block::BLANK
                }
            })
            .collect();
        self.blocks = blocks;
    }

    pub fn next_tick(&mut self) {
        match self.status {
            Status::RUNNING => {
                match self.food {
                    None => self.gen_food(),
                    Some(_i) => {}
                }
                self.update_snake();
                self.render();
            }
            _ => (),
        }
    }

    pub fn pause(&mut self) {
        self.status = Status::PENDING;
    }

    pub fn start(&mut self) {
        self.status = Status::RUNNING;
        self.render();
    }
    pub fn blocks(&self) -> *const Block {
        self.blocks.as_ptr()
    }
    pub fn set_dir_top(&mut self) {
        self.set_dir(Direction::TOP);
    }
    pub fn set_dir_right(&mut self) {
        self.set_dir(Direction::RIGHT);
    }
    pub fn set_dir_bottom(&mut self) {
        self.set_dir(Direction::BOTTOM);
    }
    pub fn set_dir_left(&mut self) {
        self.set_dir(Direction::LEFT);
    }

    pub fn snake(&self) -> *const usize {
        self.snake.as_ptr()
    }
    pub fn status(&self) -> String {
        match self.status {
            Status::OVER => "OVER".into(),
            Status::RUNNING => "RUNNING".into(),
            Status::PENDING => "PENDING".into(),
        }
    }

    pub fn new(width: usize, height: usize, walls: Vec<usize>) -> Game {
        let snake = vec![
            width / 2 + height / 2 * width - 2,
            width / 2 + height / 2 * width - 1,
            width / 2 + height / 2 * width,
            width / 2 + height / 2 * width + 1,
        ];

        let blocks: Vec<Block> = (0..width * height).map(|_index| Block::BLANK).collect();

        Game {
            width,
            height,
            blocks,
            snake,
            direction: Direction::RIGHT,
            status: Status::PENDING,
            walls,
            food: None,
        }
    }
}
