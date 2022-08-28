pub mod position;
use crate::position::Coordinates;

pub fn main() {
    let location = Coordinates {x: 2,y: 4};

    println!("{} {}", location.get_position().x,location.get_position().y);
}
