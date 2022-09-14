mod position;
mod food;
mod snake;

use crate::position::Coordinates;
use crate::food::Food;
use crate::snake::Snake;
use crate::snake::SnakeDirection;

pub fn main() {

    let location = Coordinates{_x:24,_y:25};
    println!("{} {}", location._x, location._y);

    let mut food_snake = Food::new(location, false);
    food_snake.create_food();
    println!("{} {}", food_snake._food_position._x,food_snake._food_position._y);

    println!("{:?}", food_snake.is_bonus_food());


    let mut snake = Snake::new();

    snake.init_snake();
    snake.display_snake();
    snake.crawl_snake();
    snake.display_snake();
    snake.crawl_snake();
    snake.display_snake();
    
    snake.set_direction(SnakeDirection::DOWN);
    snake.crawl_snake();
    snake.display_snake(); 

    snake.grow_snake(Coordinates { _x: 10, _y: 5 }); 
    snake.display_snake();
}