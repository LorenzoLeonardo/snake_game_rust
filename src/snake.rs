use std::io::Stdout;

use crossterm::{style::Print, ExecutableCommand};

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
    Esc,
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
        self.snake_body.push(Coordinates::new(6, 2));
        self.snake_body.push(Coordinates::new(5, 2));
        self.snake_body.push(Coordinates::new(4, 2));
        self.snake_body.push(Coordinates::new(3, 2));
        self.snake_body.push(Coordinates::new(2, 2));
        self.length = self.snake_body.len();
        self.xy_limit = xy_limit;
        self.head = self.snake_body[0];
        self.tail = self.snake_body[self.length - 1];
    }

    pub fn display_snake(&mut self, stdout: &mut Stdout) -> Result<(), Box<dyn std::error::Error>> {
        let mut i = 0;
        while i < self.snake_body.len() {
            stdout
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
            SnakeDirection::Esc => {}
        }
        self.head = self.snake_body[0];
        self.tail = self.snake_body[self.length - 1];

        self.check_body_collision();
    }

    pub fn remove_trail(&mut self, stdout: &mut Stdout) -> Result<(), Box<dyn std::error::Error>> {
        stdout
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
        self.is_alive = !self.snake_body[1..].contains(&self.head);
    }
}

#[cfg(test)]
mod test {
    use super::Snake;
    use crate::position::Coordinates;

    #[test]
    fn test_crawl_right() {
        let mut snake = Snake::new();
        let xy_limit = Coordinates::new(80, 25);

        snake.init_snake(xy_limit.to_owned());
        snake.set_direction(super::SnakeDirection::Right);
        for _n in 0..100 {
            snake.crawl_snake();
            if snake.head.x >= xy_limit.x {
                panic!("Shouldn't be greater than the X Limit.");
            }
        }
    }

    #[test]
    fn test_crawl_left() {
        let mut snake = Snake::new();
        let xy_limit = Coordinates::new(80, 25);

        snake.init_snake(xy_limit.to_owned());
        snake.set_direction(super::SnakeDirection::Left);
        for _n in 0..100 {
            snake.crawl_snake();
            if snake.head.x < 2 {
                panic!("Shouldn't be lesser than 2.");
            }
        }
    }

    #[test]
    fn test_crawl_up() {
        let mut snake = Snake::new();
        let xy_limit = Coordinates::new(80, 25);

        snake.init_snake(xy_limit.to_owned());
        snake.set_direction(super::SnakeDirection::Up);
        for _n in 0..100 {
            snake.crawl_snake();
            if snake.head.y < 2 {
                panic!("Shouldn't be lesser than 2.");
            }
        }
    }

    #[test]
    fn test_crawl_down() {
        let mut snake = Snake::new();
        let xy_limit = Coordinates::new(80, 25);

        snake.init_snake(xy_limit.to_owned());
        snake.set_direction(super::SnakeDirection::Up);
        for _n in 0..100 {
            snake.crawl_snake();
            if snake.head.y >= xy_limit.y {
                panic!("Shouldn't be greater than the Y Limit.");
            }
        }
    }
}
