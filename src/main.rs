mod position;
mod food;
mod snake;

use crate::position::Coordinates;
use crate::food::Food;
use crate::snake::Snake;
use crate::snake::SnakeDirection;
extern crate termion;
use std::{thread, time};

use device_query::{DeviceQuery, DeviceState, Keycode};

struct SnakeGame
{
    _screen_size: Coordinates,
    _dir: SnakeDirection,
}

impl SnakeGame 
{
    pub fn new (board_size: Coordinates, dir: SnakeDirection) -> SnakeGame
    {
        SnakeGame {_screen_size: board_size, _dir: dir}
    }

    pub fn run (&mut self)
    {
        let mut snake = Snake::new();
        let mut dir = SnakeDirection::RIGHT;
        let  ref_snake = &mut snake;
        let delay = time::Duration::from_millis(50);

        let mut flag: bool = true;
        let mut food = Food::new();

        self.clear();
        food.init_food(self._screen_size);
        food.create_food();
        ref_snake.init_snake(self._screen_size);
        while ref_snake._is_alive && self.listen_for_key_press()
        {
            self.draw_snake(ref_snake, self._dir.clone());
            food.display_food();
            if (ref_snake._head._x == food._food_position._x) &&
               (ref_snake._head._y == food._food_position._y) 
            {
                ref_snake.grow_snake(food._food_position.clone());
                food.create_food();
            }

            thread::sleep(delay);
        }
        self.clear();
        eprint!("{}{}",termion::cursor::Hide, termion::cursor::Goto(1,1));
    }

    fn listen_for_key_press(&mut self) -> bool
    {
        let device_state = DeviceState::new();
        let mut flag = true;
        let keys: Vec<Keycode> = device_state.get_keys();

        if !keys.is_empty(){
            for key in keys.iter() {

                if (key.to_string().as_str() == "Left") && (self._dir != SnakeDirection::RIGHT) {
                    self._dir = SnakeDirection::LEFT;
                } else if (key.to_string().as_str() == "Right") && (self._dir != SnakeDirection::LEFT) {
                    self._dir = SnakeDirection::RIGHT;
                } else if (key.to_string().as_str() == "Up") && (self._dir != SnakeDirection::DOWN) {
                    self._dir = SnakeDirection::UP;
                } else if (key.to_string().as_str() == "Down") && (self._dir != SnakeDirection::UP) {
                    self._dir = SnakeDirection::DOWN;
                } else if key.to_string().as_str() == "Escape" {
                    return false;
                }
            }
        }
        return true;
    }

    fn gotoxy(&mut self, x: i32, y: i32)
    {
        eprint!("{}", termion::cursor::Goto(x.try_into().unwrap(), y.try_into().unwrap()));
    }

    fn clear(&mut self)
    {
        eprint!("{}", termion::clear::All);
    }

    fn draw_snake(&mut self, snake: &mut Snake, dir: SnakeDirection)
    {
        self.clear();
        eprint!("{}{} ",termion::cursor::Hide, termion::cursor::Goto(snake._tail._x.try_into().unwrap(), snake._tail._y.try_into().unwrap()));
        snake.set_direction(dir.clone());
        snake.crawl_snake();
        snake.display_snake();
    }
}

pub fn main() {

    let mut main_game = SnakeGame::new(Coordinates { _x: 80, _y: 25 }, SnakeDirection::RIGHT);

    main_game.run();
}