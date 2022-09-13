mod position;
mod food;

use crate::position::Coordinates;
use crate::food::Food;

pub fn main() {
    let location = Coordinates::new(24,25);
    println!("{} {}", location._x, location._y);

    let mut food_snake = Food::new(location, false);
    food_snake.create_food();
    println!("{} {}", food_snake._food_position._x,food_snake._food_position._y);

    println!("{:?}", food_snake.is_bonus_food());
}