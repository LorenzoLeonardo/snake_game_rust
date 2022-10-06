/* Created by: Lorenzo Leonardo
 * Email: enzotechcomputersolutions@gmail.com
 * Date : September 15, 2022
 */
use crate::position::Coordinates;
use rand::Rng;

pub struct Food {
    pub _food_position: Coordinates,
    _xy_limit: Coordinates,
}

impl Food {
    // Contruct Food
    pub fn new () -> Food
    {
        Food {_food_position: Coordinates::new(0,0),
                _xy_limit: Coordinates::new(0,0)}
    }
    
    pub fn init_food(&mut self, limit: Coordinates)
    {
        self._xy_limit = limit;
    }

    pub fn create_food (&mut self)
    {
        self._food_position.x = rand::thread_rng().gen_range(2..self._xy_limit.x - 2);
        self._food_position.y = rand::thread_rng().gen_range(2..self._xy_limit.y - 2);
    }

    pub fn display_food(&mut self)
    {
        eprint!("{}",termion::cursor::Goto(self._food_position.x.try_into().unwrap(), self._food_position.y.try_into().unwrap()));
        eprint!("O");
    }

}
