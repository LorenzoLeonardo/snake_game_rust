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
}

impl SnakeGame 
{
    pub fn new (board_size: Coordinates) -> SnakeGame
    {
        SnakeGame {_screen_size: board_size}
    }

    pub fn run (&mut self)
    {
        let mut snake = Snake::new();
        let mut dir = SnakeDirection::RIGHT;
        let  ref_snake = &mut snake;

        self.clear();
        ref_snake.init_snake(self._screen_size);

        let delay = time::Duration::from_millis(50);
        let device_state = DeviceState::new();
        let mut flag: bool = true;
    
        while ref_snake._is_alive && flag
        {
            let keys: Vec<Keycode> = device_state.get_keys();
            if !keys.is_empty(){
                for key in keys.iter() {
                    match key.to_string().as_str() {
                        "Left" => dir = SnakeDirection::LEFT,
                        "Right" => dir = SnakeDirection::RIGHT,
                        "Up" => dir = SnakeDirection::UP,
                        "Down" => dir = SnakeDirection::DOWN,
                        "Escape" => flag = false,
                        
                        _ => println!("{}", key.to_string()),
                    }
                }
            }
            self.draw_snake(ref_snake, dir.clone());
            thread::sleep(delay);
        }
        self.clear();
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
        snake.display_snake();
        snake.set_direction(dir.clone());
        snake.crawl_snake();

/*        self.gotoxy(snake._tail._x, snake._tail._y);
        print!("{}", ' ');

        //gotoxy((m_screenSize.X/2) - ((int)m_nameLabel.length()/2), m_screenSize.Y + 1);
        //print!("%s", m_nameLabel.c_str());

        self.gotoxy(1,1);
        print!("╔");
        self.gotoxy(self._screen_size._x,1);
        print!("╗");
        self.gotoxy(1,self._screen_size._y);
        print!("╚");
        self.gotoxy(self._screen_size._x,self._screen_size._y);
        print!("╝");
        
        let mut y: i32 = 2;
        while y < self._screen_size._y
        {
            self.gotoxy(1,y);
            print!("║");

            self.gotoxy(self._screen_size._x,y);
            print!("║");
            y += 1;
        }

        let mut x: i32 = 2;
        while x < (self._screen_size._x)
        {
            self.gotoxy(x,1);
            print!("═");

            self.gotoxy(x,self._screen_size._y);
            print!("═");
            x += 1;
        }
        snake.set_direction(dir);
        snake.crawl_snake();
        let mut i = 0;
        while i < snake._length
        {
            self.gotoxy(snake._snake_body[i]._x, snake._snake_body[i]._y);
            print!("0");
            i += 1;
        }*/
    }
}

pub fn main() {

    let mut main_game = SnakeGame::new(Coordinates { _x: 80, _y: 25 });

    main_game.run();
}