use crate::position::Coordinates;
use rand::Rng;

pub struct Food {
    pub _food_position: Coordinates,
    _is_bonus: bool,
}

impl Food {
    // Contruct Food
    pub fn new (food_position: Coordinates, is_bonus: bool) -> Food {
        Food {_food_position: food_position, _is_bonus: is_bonus}
    }

    pub fn create_food (&mut self) {
        self._food_position._x = rand::thread_rng().gen_range(0..100);
        self._food_position._y = rand::thread_rng().gen_range(0..100);
    }

    pub fn is_bonus_food(&self) -> bool {
       self._is_bonus
    }
}
