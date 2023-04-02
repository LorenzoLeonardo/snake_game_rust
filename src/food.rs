/* Created by: Lorenzo Leonardo
 * Email: enzotechcomputersolutions@gmail.com
 * Date : September 15, 2022
 */
// 3rd party crates
use rand::Rng;
// My crates
use crate::position::Coordinates;

pub struct Food {
    pub food_position: Coordinates,
    upper_left: Coordinates,
    bottom_right: Coordinates,
}

impl Food {
    // Contruct Food
    pub fn new(upper_left: Coordinates, bottom_right: Coordinates) -> Self {
        Self {
            food_position: Coordinates::new(0, 0),
            upper_left,
            bottom_right,
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

    pub fn display_food<C>(&mut self, render_food: C)
    where
        C: FnOnce(&Coordinates),
    {
        render_food(&self.food_position);
    }
}
