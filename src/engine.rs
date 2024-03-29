/* Created by: Lorenzo Leonardo
 * Email: enzotechcomputersolutions@gmail.com
 * Date : September 15, 2022
 */
// Standard libraries
use std::time::Duration;
// 3rd party crates
use crossterm::event::KeyCode;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
// My crates
use crate::draw::Draw;
use crate::food::Food;
use crate::position::Coordinates;
use crate::snake::{CurrentKeyPressed, Snake};

#[derive(Debug)]
pub enum SnakeGameState {
    SnakeDied,
    Quit,
}

pub struct GameEngine {
    upper_left: Coordinates,
    bottom_right: Coordinates,
    current_key_pressed: CurrentKeyPressed,
    rx_key_event: UnboundedReceiver<KeyCode>,
    tx_game_state: UnboundedSender<SnakeGameState>,
}

impl GameEngine {
    pub fn new(
        rx_key_event: UnboundedReceiver<KeyCode>,
        tx_game_state: UnboundedSender<SnakeGameState>,
    ) -> Self {
        // Initialize the board size
        let upper_left = Coordinates::new(1, 3);
        let bottom_right = Coordinates::new(120, 37);
        let current_key_pressed = CurrentKeyPressed::Right;
        Self {
            upper_left,
            bottom_right,
            current_key_pressed,
            rx_key_event,
            tx_game_state,
        }
    }

    fn listen_for_key_press(&mut self) -> CurrentKeyPressed {
        match self.rx_key_event.try_recv() {
            Ok(key) => {
                if key == KeyCode::Up && self.current_key_pressed != CurrentKeyPressed::Down {
                    CurrentKeyPressed::Up
                } else if key == KeyCode::Down && self.current_key_pressed != CurrentKeyPressed::Up
                {
                    CurrentKeyPressed::Down
                } else if key == KeyCode::Left
                    && self.current_key_pressed != CurrentKeyPressed::Right
                {
                    CurrentKeyPressed::Left
                } else if key == KeyCode::Right
                    && self.current_key_pressed != CurrentKeyPressed::Left
                {
                    CurrentKeyPressed::Right
                } else if key == KeyCode::Esc {
                    CurrentKeyPressed::Esc
                } else {
                    self.current_key_pressed
                }
            }
            Err(_e) => self.current_key_pressed,
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialization
        let mut draw = Draw::init()?;
        let mut snake = Snake::new(self.upper_left, self.bottom_right);
        let mut food = Food::new(self.upper_left, self.bottom_right);
        let delay = Duration::from_millis(30);

        // Initialize
        self.initialize(&mut draw, &snake, &mut food)?;

        // game loop
        while snake.is_alive {
            // input
            self.current_key_pressed = self.listen_for_key_press();
            if self.current_key_pressed == CurrentKeyPressed::Esc {
                break;
            }
            // update
            self.update(&mut snake, &mut food);
            // render
            self.render(&snake, &food, &mut draw);

            tokio::time::sleep(delay).await;
        }
        // Shutdown
        draw.deinit()?;

        if self.current_key_pressed == CurrentKeyPressed::Esc {
            self.shutdown(SnakeGameState::Quit).await;
        } else {
            self.shutdown(SnakeGameState::SnakeDied).await;
        }

        Ok(())
    }

    fn render(&self, snake: &Snake, food: &Food, draw: &mut Draw) {
        snake.erase_trail(|body_trail| {
            draw.remove_snake_trail(body_trail);
        });
        snake.display_snake(|snake_body| {
            draw.draw_snake(snake_body, std::borrow::Cow::Owned("█"));
        });
        food.display_food(|food_position| {
            draw.draw_food(food_position, std::borrow::Cow::Owned("@"));
        });
    }

    fn update(&mut self, snake: &mut Snake, food: &mut Food) {
        if snake.head == food.food_position {
            snake.grow_snake(food.food_position);
            food.create_food(&snake.snake_body);
        }
        snake.set_current_key_pressed(self.current_key_pressed);
        snake.crawl_snake();
    }

    fn initialize(
        &self,
        draw: &mut Draw,
        snake: &Snake,
        food: &mut Food,
    ) -> Result<(), Box<dyn std::error::Error>> {
        draw.draw_board(&self.upper_left, &self.bottom_right)?;
        food.create_food(&snake.snake_body);
        Ok(())
    }

    async fn shutdown(&mut self, state: SnakeGameState) {
        if let Err(err) = self.tx_game_state.send(state) {
            eprintln!("Error: {:?}", err);
        }
    }
}
