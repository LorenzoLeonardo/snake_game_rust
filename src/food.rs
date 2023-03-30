use std::io::Stdout;

/* Created by: Lorenzo Leonardo
 * Email: enzotechcomputersolutions@gmail.com
 * Date : September 15, 2022
 */
use crate::position::Coordinates;
use crossterm::{style::Print, ExecutableCommand};
use rand::Rng;

pub struct Food {
    pub food_position: Coordinates,
    xy_limit: Coordinates,
}

impl Food {
    // Contruct Food
    pub fn new() -> Self {
        Self {
            food_position: Coordinates::new(0, 0),
            xy_limit: Coordinates::new(0, 0),
        }
    }

    pub fn init_food(&mut self, limit: Coordinates) {
        self.xy_limit = limit;
    }

    pub fn create_food(&mut self, snake_body: &[Coordinates]) {
        self.food_position.x = rand::thread_rng().gen_range(2..self.xy_limit.x - 2);
        self.food_position.y = rand::thread_rng().gen_range(2..self.xy_limit.y - 2);

        // Create the food must not be at the location of the snake body
        while snake_body.contains(&self.food_position) {
            self.food_position.x = rand::thread_rng().gen_range(2..self.xy_limit.x - 2);
            self.food_position.y = rand::thread_rng().gen_range(2..self.xy_limit.y - 2);
        }
    }

    pub fn display_food(&mut self, mut stdout: &Stdout) -> Result<(), Box<dyn std::error::Error>> {
        stdout
            .execute(crossterm::cursor::MoveTo(
                self.food_position.x,
                self.food_position.y,
            ))?
            .execute(Print("O"))?;

        Ok(())
    }
}
