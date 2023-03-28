/* Created by: Lorenzo Leonardo
 * Email: enzotechcomputersolutions@gmail.com
 * Date : September 15, 2022
 */
mod food;
mod position;
mod snake;

use std::io::stdout;
use std::io::Stdout;
use std::time;

use crossterm::cursor::Hide;
use crossterm::cursor::Show;
use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal;
use crossterm::ExecutableCommand;
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::food::Food;
use crate::position::Coordinates;
use crate::snake::Snake;
use crate::snake::SnakeDirection;

struct SnakeGame {
    screen_size: Coordinates,
    dir: SnakeDirection,
    rx: UnboundedReceiver<KeyCode>,
}

impl SnakeGame {
    pub fn new(
        screen_size: Coordinates,
        dir: SnakeDirection,
        rx: UnboundedReceiver<KeyCode>,
    ) -> Self {
        Self {
            screen_size,
            dir,
            rx,
        }
    }

    pub fn run(&mut self, stdout: &mut Stdout) -> Result<(), Box<dyn std::error::Error>> {
        let mut snake = Snake::new();
        let mut food = Food::new();
        let ref_snake = &mut snake;
        let ref_food = &mut food;
        let delay = time::Duration::from_millis(50);

        ref_food.init_food(self.screen_size);
        ref_food.create_food();
        ref_snake.init_snake(self.screen_size);
        while ref_snake.is_alive {
            self.dir = match self.rx.try_recv() {
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
                        break;
                    } else {
                        self.dir
                    }
                }
                Err(_e) => self.dir,
            };

            self.clear(stdout)?;
            self.draw_snake(ref_snake, self.dir, stdout)?;
            self.draw_food(ref_food, stdout)?;

            if ref_snake.head == ref_food.food_position {
                ref_snake.grow_snake(ref_food.food_position);
                ref_food.create_food();
            }

            std::thread::sleep(delay);
        }
        Ok(())
    }

    fn clear(&mut self, stdout: &mut Stdout) -> Result<(), Box<dyn std::error::Error>> {
        stdout.execute(Hide)?;
        Ok(())
    }

    fn draw_snake(
        &mut self,
        snake: &mut Snake,
        dir: SnakeDirection,
        stdout: &mut Stdout,
    ) -> Result<(), Box<dyn std::error::Error>> {
        snake.remove_trail(stdout)?;
        snake.set_direction(dir);
        snake.crawl_snake();
        snake.display_snake(stdout)?;
        Ok(())
    }

    fn draw_food(
        &mut self,
        food: &mut Food,
        stdout: &Stdout,
    ) -> Result<(), Box<dyn std::error::Error>> {
        food.display_food(stdout)?;
        Ok(())
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = unbounded_channel();

    let mut stdout = stdout();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(Hide)?;

    std::thread::spawn(move || loop {
        if let Event::Key(key_event) = read().unwrap() {
            match key_event.code {
                KeyCode::Down => {
                    tx.send(KeyCode::Down).unwrap();
                }
                KeyCode::Up => {
                    tx.send(KeyCode::Up).unwrap();
                }
                KeyCode::Left => {
                    tx.send(KeyCode::Left).unwrap();
                }
                KeyCode::Right => {
                    tx.send(KeyCode::Right).unwrap();
                }
                KeyCode::Esc => {
                    tx.send(KeyCode::Esc).unwrap();
                    break;
                }
                _ => {}
            }
        }
    });

    let mut main_game = SnakeGame::new(Coordinates::new(80, 25), SnakeDirection::Right, rx);

    main_game.run(&mut stdout)?;

    stdout
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(Show)?;

    Ok(())
}
