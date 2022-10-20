/* Created by: Lorenzo Leonardo
 * Email: enzotechcomputersolutions@gmail.com
 * Date : September 15, 2022
 */
use crate::position::Coordinates;

#[derive(PartialEq)]
pub enum SnakeDirection {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}
impl Copy for SnakeDirection {}
impl Clone for SnakeDirection {
    fn clone(&self) -> Self {
        *self
    }
}

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
            direction: SnakeDirection::RIGHT,
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

    pub fn display_snake(&mut self) {
        let mut i = 0;
        while i < self.snake_body.len() {
            eprint!(
                "{}",
                termion::cursor::Goto(
                    self.snake_body[i].x.try_into().unwrap(),
                    self.snake_body[i].y.try_into().unwrap()
                )
            );
            eprint!("@");
            i += 1;
        }
    }

    pub fn crawl_snake(&mut self) {
        match self.direction {
            SnakeDirection::RIGHT => self.crawl_right(),
            SnakeDirection::LEFT => self.crawl_left(),
            SnakeDirection::UP => self.crawl_up(),
            SnakeDirection::DOWN => self.crawl_down(),
        }
        self.head = self.snake_body[0];
        self.tail = self.snake_body[self.length - 1];

        self.check_body_collision();
    }

    pub fn remove_trail(&mut self) {
        eprint!(
            "{} ",
            termion::cursor::Goto(
                self.tail.x.try_into().unwrap(),
                self.tail.y.try_into().unwrap()
            )
        );
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
