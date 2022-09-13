pub struct Coordinates {
    pub _x: i32,
    pub _y: i32,
}

impl Coordinates {
    // Contruct Coordinates
    pub fn new (x: i32, y: i32) -> Coordinates {
        Coordinates {_x: x, _y: y}
    }
}
