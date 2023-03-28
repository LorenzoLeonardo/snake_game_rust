use std::io::Stdout;

use crossterm::{cursor::Hide, style::Print, ExecutableCommand};

/* Created by: Lorenzo Leonardo
 * Email: enzotechcomputersolutions@gmail.com
 * Date : September 15, 2022
 */
use crate::position::Coordinates;

#[derive(PartialEq, Copy, Clone)]
pub enum SnakeDirection {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone)]
pub struct Snake {
    pub snake_body: Vec<Coordinates>,
    pub head: Coordinates,
    pub tail: Coordinates,
    pub direction: SnakeDirection,
    pub length: usize,
    pub is_alive: bool,
    pub xy_limit: Coordinates,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            snake_body: Vec::new(),
            head: Coordinates::new(0, 0),
            tail: Coordinates::new(0, 0),
            direction: SnakeDirection::Right,
            length: 0,
            is_alive: true,
            xy_limit: Coordinates::new(0, 0),
        }
    }

    pub fn init_snake(&mut self, xy_limit: Coordinates) {
        self.snake_body.push(Coordinates::new(5, 2));
        self.snake_body.push(Coordinates::new(4, 2));
        self.snake_body.push(Coordinates::new(3, 2));
        self.snake_body.push(Coordinates::new(2, 2));
        self.snake_body.push(Coordinates::new(1, 2));
        self.length = self.snake_body.len();
        self.xy_limit = xy_limit;
        self.head = self.snake_body[0];
        self.tail = self.snake_body[self.length - 1];
    }

    pub fn display_snake(&mut self, stdout: &mut Stdout) -> Result<(), Box<dyn std::error::Error>> {
        let mut i = 0;
        while i < self.snake_body.len() {
            stdout
                .execute(Hide)?
                .execute(crossterm::cursor::MoveTo(
                    self.snake_body[i].x,
                    self.snake_body[i].y,
                ))?
                .execute(Print("@"))?;

            i += 1;
        }
        Ok(())
    }

    pub fn crawl_snake(&mut self) {
        match self.direction {
            SnakeDirection::Right => self.crawl_right(),
            SnakeDirection::Left => self.crawl_left(),
            SnakeDirection::Up => self.crawl_up(),
            SnakeDirection::Down => self.crawl_down(),
        }
        self.head = self.snake_body[0];
        self.tail = self.snake_body[self.length - 1];

        self.check_body_collision();
    }

    pub fn remove_trail(&mut self, stdout: &mut Stdout) -> Result<(), Box<dyn std::error::Error>> {
        stdout
            .execute(Hide)?
            .execute(crossterm::cursor::MoveTo(self.tail.x, self.tail.y))?
            .execute(Print(" "))?;
        Ok(())
    }

    pub fn set_direction(&mut self, dir: SnakeDirection) {
        self.direction = dir;
    }

    pub fn grow_snake(&mut self, pos: Coordinates) {
        self.snake_body.push(pos);
        self.tail = pos;
        self.length = self.snake_body.len();
    }

    fn crawl_right(&mut self) {
        let mut i = self.length - 1;
        while i > 0 {
            self.snake_body[i] = self.snake_body[i - 1];
            i -= 1;
        }
        self.snake_body[0].x += 1;
        if self.snake_body[0].x >= self.xy_limit.x {
            self.snake_body[0].x = 2;
        }
    }

    fn crawl_left(&mut self) {
        let mut i = self.length - 1;
        while i > 0 {
            self.snake_body[i] = self.snake_body[i - 1];
            i -= 1;
        }
        self.snake_body[0].x -= 1;
        if self.snake_body[0].x < 2 {
            self.snake_body[0].x = self.xy_limit.x - 1;
        }
    }

    fn crawl_up(&mut self) {
        let mut i = self.length - 1;
        while i > 0 {
            self.snake_body[i] = self.snake_body[i - 1];
            i -= 1;
        }
        self.snake_body[0].y -= 1;
        if self.snake_body[0].y < 2 {
            self.snake_body[0].y = self.xy_limit.y - 1;
        }
    }

    fn crawl_down(&mut self) {
        let mut i = self.length - 1;
        while i > 0 {
            self.snake_body[i] = self.snake_body[i - 1];
            i -= 1;
        }
        self.snake_body[0].y += 1;
        if self.snake_body[0].y >= self.xy_limit.y {
            self.snake_body[0].y = 2;
        }
    }

    fn check_body_collision(&mut self) {
        let mut i = 1;
        while i < self.length {
            if self.snake_body[i] == self.head {
                self.is_alive = false;
                break;
            }
            i += 1;
        }
    }
}
