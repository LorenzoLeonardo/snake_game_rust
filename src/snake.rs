/* Created by: Lorenzo Leonardo
 * Email: enzotechcomputersolutions@gmail.com
 * Date : September 15, 2022
 */
// My crates
use crate::position::Coordinates;

#[derive(PartialEq, Copy, Clone)]
pub enum CurrentKeyPressed {
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
    pub current_key_pressed: CurrentKeyPressed,
    pub length: usize,
    pub is_alive: bool,
    pub upper_left: Coordinates,
    pub bottom_right: Coordinates,
    pub trail: Coordinates,
}

impl Snake {
    pub fn new(upper_left: Coordinates, bottom_right: Coordinates) -> Self {
        // Starting positon of the snake must be at the top most part of the board going right
        let snake_body = vec![
            Coordinates::new(upper_left.x + 5, upper_left.y + 1),
            Coordinates::new(upper_left.x + 4, upper_left.y + 1),
            Coordinates::new(upper_left.x + 3, upper_left.y + 1),
            Coordinates::new(upper_left.x + 2, upper_left.y + 1),
            Coordinates::new(upper_left.x + 1, upper_left.y + 1),
        ];
        let length = snake_body.len();
        let head = snake_body[0];
        let tail = snake_body[length - 1];
        let trail = Coordinates::new(upper_left.x, upper_left.y + 1);
        Self {
            snake_body,
            head,
            tail,
            current_key_pressed: CurrentKeyPressed::Right,
            length,
            is_alive: true,
            upper_left,
            bottom_right,
            trail,
        }
    }

    pub fn display_snake<C>(&self, render_snake_body: C)
    where
        C: FnOnce(&Vec<Coordinates>),
    {
        render_snake_body(&self.snake_body);
    }

    pub fn erase_trail<C>(&self, erase_snake_trail: C)
    where
        C: FnOnce(&Coordinates),
    {
        erase_snake_trail(&self.trail);
    }

    pub fn crawl_snake(&mut self) {
        match self.current_key_pressed {
            CurrentKeyPressed::Right => self.crawl_right(),
            CurrentKeyPressed::Left => self.crawl_left(),
            CurrentKeyPressed::Up => self.crawl_up(),
            CurrentKeyPressed::Down => self.crawl_down(),
            CurrentKeyPressed::Esc => {}
        }
        self.head = self.snake_body[0];
        self.trail = self.tail;
        self.tail = self.snake_body[self.length - 1];

        self.check_body_collision();
    }
    pub fn set_current_key_pressed(&mut self, current_key_pressed: CurrentKeyPressed) {
        self.current_key_pressed = current_key_pressed;
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
        if self.snake_body[0].x >= self.bottom_right.x {
            self.snake_body[0].x = self.upper_left.x + 1;
        }
    }

    fn crawl_left(&mut self) {
        let mut i = self.length - 1;
        while i > 0 {
            self.snake_body[i] = self.snake_body[i - 1];
            i -= 1;
        }
        self.snake_body[0].x -= 1;
        if self.snake_body[0].x <= self.upper_left.x {
            self.snake_body[0].x = self.bottom_right.x - 1;
        }
    }

    fn crawl_up(&mut self) {
        let mut i = self.length - 1;
        while i > 0 {
            self.snake_body[i] = self.snake_body[i - 1];
            i -= 1;
        }
        self.snake_body[0].y -= 1;
        if self.snake_body[0].y <= self.upper_left.y {
            self.snake_body[0].y = self.bottom_right.y - 1;
        }
    }

    fn crawl_down(&mut self) {
        let mut i = self.length - 1;
        while i > 0 {
            self.snake_body[i] = self.snake_body[i - 1];
            i -= 1;
        }
        self.snake_body[0].y += 1;
        if self.snake_body[0].y >= self.bottom_right.y {
            self.snake_body[0].y = self.upper_left.y + 1;
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
        let upper_left = Coordinates::new(1, 1);
        let bottom_right = Coordinates::new(80, 25);
        let mut snake = Snake::new(upper_left, bottom_right);

        snake.set_current_key_pressed(super::CurrentKeyPressed::Right);
        for _n in 0..100 {
            snake.crawl_snake();
            if snake.head.x >= bottom_right.x {
                panic!("Snake head should not collide at the right side of the board.");
            }
        }
    }

    #[test]
    fn test_crawl_left() {
        let upper_left = Coordinates::new(1, 1);
        let bottom_right = Coordinates::new(80, 25);
        let mut snake = Snake::new(upper_left, bottom_right);

        snake.set_current_key_pressed(super::CurrentKeyPressed::Left);
        for _n in 0..100 {
            snake.crawl_snake();
            if snake.head.x <= upper_left.x {
                panic!("Snake head should not collide at the left side of the board.");
            }
        }
    }

    #[test]
    fn test_crawl_up() {
        let upper_left = Coordinates::new(1, 1);
        let bottom_right = Coordinates::new(80, 25);
        let mut snake = Snake::new(upper_left, bottom_right);

        snake.set_current_key_pressed(super::CurrentKeyPressed::Up);
        for _n in 0..100 {
            snake.crawl_snake();
            if snake.head.y <= upper_left.y {
                panic!("Snake head should not collide at the upper side of the board.");
            }
        }
    }

    #[test]
    fn test_crawl_down() {
        let upper_left = Coordinates::new(1, 1);
        let bottom_right = Coordinates::new(80, 25);
        let mut snake = Snake::new(upper_left, bottom_right);

        snake.set_current_key_pressed(super::CurrentKeyPressed::Up);
        for _n in 0..100 {
            snake.crawl_snake();
            if snake.head.y >= bottom_right.y {
                panic!("Snake head should not collide at the lower side of the board.");
            }
        }
    }
}
