/* Created by: Lorenzo Leonardo
 * Email: enzotechcomputersolutions@gmail.com
 * Date : September 15, 2022
 */
// Standard libraries
use std::io::Stdout;
use std::time::Duration;
// 3rd party crates
use crossterm::event::KeyCode;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
// My crates
use crate::draw::Draw;
use crate::food::Food;
use crate::position::Coordinates;
use crate::snake::{Snake, SnakeDirection};

pub struct GameEngine {
    upper_left: Coordinates,
    bottom_right: Coordinates,
    dir: SnakeDirection,
    rx_key_event: UnboundedReceiver<KeyCode>,
    tx_snake_died: UnboundedSender<bool>,
}

impl GameEngine {
    pub fn new(
        upper_left: Coordinates,
        bottom_right: Coordinates,
        dir: SnakeDirection,
        rx_key_event: UnboundedReceiver<KeyCode>,
        tx_snake_died: UnboundedSender<bool>,
    ) -> Self {
        Self {
            upper_left,
            bottom_right,
            dir,
            rx_key_event,
            tx_snake_died,
        }
    }
    fn listen_for_key_press(&mut self) -> SnakeDirection {
        match self.rx_key_event.try_recv() {
            Ok(key) => {
                if key == KeyCode::Up && self.dir != SnakeDirection::Down {
                    SnakeDirection::Up
                } else if key == KeyCode::Down && self.dir != SnakeDirection::Up {
                    SnakeDirection::Down
                } else if key == KeyCode::Left && self.dir != SnakeDirection::Right {
                    SnakeDirection::Left
                } else if key == KeyCode::Right && self.dir != SnakeDirection::Left {
                    SnakeDirection::Right
                } else if key == KeyCode::Esc {
                    SnakeDirection::Esc
                } else {
                    self.dir
                }
            }
            Err(_e) => self.dir,
        }
    }
    /// The game loop
    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialization
        let mut stdout = Draw::initialize_terminal()?;

        Draw::draw_board(&mut stdout, &self.upper_left, &self.bottom_right)?;

        let mut snake = Snake::new(self.upper_left, self.bottom_right);
        let mut food = Food::new(self.upper_left, self.bottom_right);
        let delay = Duration::from_millis(30);

        food.create_food(&snake.snake_body);

        while snake.is_alive {
            // input
            self.dir = self.listen_for_key_press();
            if self.dir == SnakeDirection::Esc {
                break;
            }

            // update
            if snake.head == food.food_position {
                snake.grow_snake(food.food_position);
                food.create_food(&snake.snake_body);
            }

            // render
            self.draw_snake(&mut snake, self.dir, &mut stdout);
            self.draw_food(&mut food, &mut stdout);

            tokio::time::sleep(delay).await;
        }

        // Shutdown
        if let Err(_err) = self.tx_snake_died.send(true) {}

        Draw::restore_terminal(stdout)
    }

    fn draw_snake(&mut self, snake: &mut Snake, dir: SnakeDirection, stdout: &mut Stdout) {
        snake.remove_trail(|body_trail| {
            Draw::remove_snake_trail(stdout, body_trail);
        });
        snake.set_direction(dir);
        snake.crawl_snake();
        snake.display_snake(|snake_body| {
            Draw::draw_snake(stdout, snake_body, std::borrow::Cow::Owned("█"));
        });
    }

    fn draw_food(&mut self, food: &mut Food, stdout: &mut Stdout) {
        food.display_food(|food_position| {
            Draw::draw_food(stdout, food_position, std::borrow::Cow::Owned("@"));
        });
    }
}
