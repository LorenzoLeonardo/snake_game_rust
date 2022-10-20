/* Created by: Lorenzo Leonardo
 * Email: enzotechcomputersolutions@gmail.com
 * Date : September 15, 2022
 */
mod food;
mod position;
mod snake;

use crate::food::Food;
use crate::position::Coordinates;
use crate::snake::Snake;
use crate::snake::SnakeDirection;
extern crate termion;
use std::{thread, time};

use device_query::{DeviceQuery, DeviceState, Keycode};

struct SnakeGame {
    screen_size: Coordinates,
    dir: SnakeDirection,
}

impl SnakeGame {
    pub fn new(board_size: Coordinates, dir: SnakeDirection) -> Self {
        Self {
            screen_size: board_size,
            dir: dir,
        }
    }

    pub fn run(&mut self) {
        let mut snake = Snake::new();
        let mut food = Food::new();
        let ref_snake = &mut snake;
        let ref_food = &mut food;
        let delay = time::Duration::from_millis(35);

        ref_food.init_food(self.screen_size);
        ref_food.create_food();
        ref_snake.init_snake(self.screen_size);
        while ref_snake.is_alive && self.listen_for_key_press() {
            self.clear();
            self.draw_snake(ref_snake, self.dir);
            self.draw_food(ref_food);

            if ref_snake.head == ref_food.food_position {
                ref_snake.grow_snake(ref_food.food_position);
                ref_food.create_food();
            }

            thread::sleep(delay);
        }
    }

    fn listen_for_key_press(&mut self) -> bool {
        let device_state = DeviceState::new();
        let keys: Vec<Keycode> = device_state.get_keys();

        if !keys.is_empty() {
            for key in keys.iter() {
                if (key.to_string().as_str() == "Left") && (self.dir != SnakeDirection::RIGHT) {
                    self.dir = SnakeDirection::LEFT;
                } else if (key.to_string().as_str() == "Right")
                    && (self.dir != SnakeDirection::LEFT)
                {
                    self.dir = SnakeDirection::RIGHT;
                } else if (key.to_string().as_str() == "Up") && (self.dir != SnakeDirection::DOWN) {
                    self.dir = SnakeDirection::UP;
                } else if (key.to_string().as_str() == "Down") && (self.dir != SnakeDirection::UP) {
                    self.dir = SnakeDirection::DOWN;
                } else if key.to_string().as_str() == "Escape" {
                    return false;
                }
            }
        }
        return true;
    }

    fn clear(&mut self) {
        eprint!("{}", termion::clear::All);
    }

    fn draw_snake(&mut self, snake: &mut Snake, dir: SnakeDirection) {
        snake.remove_trail();
        snake.set_direction(dir);
        snake.crawl_snake();
        snake.display_snake();
    }

    fn draw_food(&mut self, food: &mut Food) {
        food.display_food();
    }
}

pub fn main() {
    let mut main_game = SnakeGame::new(Coordinates::new(80, 25), SnakeDirection::RIGHT);

    eprint!("{}", termion::clear::All);
    eprint!("{}", termion::cursor::Hide);
    main_game.run();
    eprint!("{}", termion::clear::All);
    eprint!("{}", termion::cursor::Restore);
    eprint!("{}", termion::cursor::Goto(1, 1));
}
