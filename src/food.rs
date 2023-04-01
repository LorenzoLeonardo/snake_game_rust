use std::{borrow::Cow, io::Stdout};

/* Created by: Lorenzo Leonardo
 * Email: enzotechcomputersolutions@gmail.com
 * Date : September 15, 2022
 */
use crate::position::Coordinates;
use crossterm::{style::Print, ExecutableCommand};
use rand::Rng;

pub struct Food<'a> {
    pub food_position: Coordinates,
    upper_left: Coordinates,
    bottom_right: Coordinates,
    food_style: Cow<'a, &'a str>,
}

impl<'a> Food<'a> {
    // Contruct Food
    pub fn new(
        upper_left: Coordinates,
        bottom_right: Coordinates,
        food_style: Cow<'a, &'a str>,
    ) -> Self {
        Self {
            food_position: Coordinates::new(0, 0),
            upper_left,
            bottom_right,
            food_style,
        }
    }

    pub fn create_food(&mut self, snake_body: &[Coordinates]) {
        // Food must be inside the board
        self.food_position.x =
            rand::thread_rng().gen_range((self.upper_left.x + 1)..(self.bottom_right.x - 1));
        self.food_position.y =
            rand::thread_rng().gen_range((self.upper_left.y + 1)..(self.bottom_right.y - 1));

        // Create the food must not be at the location of the snake body
        while snake_body.contains(&self.food_position) {
            self.food_position.x =
                rand::thread_rng().gen_range((self.upper_left.x + 1)..(self.bottom_right.x - 1));
            self.food_position.y =
                rand::thread_rng().gen_range((self.upper_left.y + 1)..(self.bottom_right.y - 1));
        }
    }

    pub fn display_food(&mut self, mut stdout: &Stdout) -> Result<(), Box<dyn std::error::Error>> {
        stdout
            .execute(crossterm::cursor::MoveTo(
                self.food_position.x,
                self.food_position.y,
            ))?
            .execute(Print(&self.food_style))?;

        Ok(())
    }
}
