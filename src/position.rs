
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}


impl Coordinates {
    pub fn get_position(&self) -> &Coordinates {
        self 
    }
}
