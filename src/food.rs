use crate::position::Coordinates;
use rand::Rng;

pub struct Food {
    pub _food_position: Coordinates,
    _is_bonus: bool,
    _xy_limit: Coordinates,
}

impl Food {
    // Contruct Food
    pub fn new () -> Food {
        Food {_food_position: Coordinates{_x:0, _y:0}, _is_bonus: false, _xy_limit: Coordinates { _x: 0, _y: 0 } }
    }
    
    pub fn init_food(&mut self, limit: Coordinates) {
        self._xy_limit = limit.clone();
    }

    pub fn create_food (&mut self) {
        self._food_position._x = rand::thread_rng().gen_range(2..self._xy_limit._x - 2);
        self._food_position._y = rand::thread_rng().gen_range(2..self._xy_limit._y - 2);
    }

    pub fn is_bonus_food(&self) -> bool {
       self._is_bonus
    }

    pub fn display_food(&mut self)
    {
        eprint!("{}",termion::cursor::Goto(self._food_position._x.try_into().unwrap(), self._food_position._y.try_into().unwrap()));
        eprint!("O");
    }

}
