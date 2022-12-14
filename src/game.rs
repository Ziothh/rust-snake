use piston_window::*;
use piston_window::types::Color;

use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng,};


use crate::snake::{Direction, Snake};
use crate::helpers::draw::{draw_block, draw_rect};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEWAIT_COLOR: Color = [0.00, 0.00, 0.00, 0.7];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_started: bool,
    game_over: bool,
    waiting_time: f64,

    rng: ThreadRng,
}


impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        return Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_started: false,
            game_over: false,
            rng: thread_rng()
        };
    }

    fn get_random_pos(&mut self) -> (i32, i32) {
        /* 
        There is still a small bug where on the start of the game
        the snake could be placed on the same tile as the food.
        */ 
        return (
            self.rng.gen_range(1..(self.width - 1)), 
            self.rng.gen_range((1..(self.width - 1)))
        )
    }

    pub fn key_pressed(&mut self, key: Key) {
        if (self.game_over) { return; }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            Key::Z => Some(Direction::Up),
            Key::S => Some(Direction::Down),
            Key::Q => Some(Direction::Left),
            Key::D => Some(Direction::Right),
            _ => None,
        };

        if (dir.is_none()) {return;}

        if (!self.game_started) {
            self.game_started = true;
        } else if (dir.unwrap() == self.snake.get_head_direction().opposite()) {return;}

        self.update_snake(dir);
    }

    fn add_food(&mut self) {
        let (mut new_x, mut new_y) = self.get_random_pos();

        while self.snake.overlap_tail(new_x, new_y) {
            (new_x, new_y) = self.get_random_pos();
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    fn update_snake(&mut self, direction: Option<Direction>) {
        if (self.check_if_snake_alive(direction)) {
            self.snake.move_forward(direction);
            self.check_eating();
        } else {
            self.game_over = true;
        }

        self.waiting_time = 0.0;
    }    

    pub fn draw(&self, ctx: &Context, g2d: &mut G2d) {
        self.snake.draw(ctx, g2d);

        if (self.food_exists) {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, ctx, g2d);
        }

        draw_rect(BORDER_COLOR, 0, 0, self.width, 1, ctx, g2d);
        draw_rect(BORDER_COLOR, 0, self.height - 1, self.width, 1, ctx, g2d);
        draw_rect(BORDER_COLOR, 0, 0, 1, self.height, ctx, g2d);
        draw_rect(BORDER_COLOR, self.width - 1, 0, 1, self.height, ctx, g2d);

        if (self.game_over) {
            draw_rect(GAMEOVER_COLOR, 0, 0, self.width, self.height, ctx, g2d);
        }
        else if (!self.game_started) {
            draw_rect(GAMEWAIT_COLOR, 0, 0, self.width, self.height, ctx, g2d);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if (self.game_over) {
            if (self.waiting_time > RESTART_TIME) {
                self.reset();
            }

            return;
        }

        if (!self.game_started) {
            return;
        }

        if (!self.food_exists) {
            self.add_food();
        }

        if (self.waiting_time > MOVING_PERIOD) {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.get_head_position();

        if (self.food_exists && self.food_x == head_x && self.food_y == head_y) {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn check_if_snake_alive(&self, direction: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(direction);

        // Check tail
        if (self.snake.overlap_tail(next_x, next_y)) {
            return false;
        }

        // Check borders
        return next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn reset(&mut self) {
        let (snake_x, snake_y) = self.get_random_pos();
        self.snake = Snake::new(snake_x, snake_y);
        self.waiting_time = 0.0;
        self.food_exists = true;
        (self.food_x, self.food_y) = self.get_random_pos();
        self.game_over = false;
        self.game_started = false;
    }
}