#[derive(PartialEq)]
pub struct Coordinates {
    pub _x: i32,
    pub _y: i32,
}
impl Copy for Coordinates {}
impl Clone for Coordinates {
    fn clone(&self) -> Self {
        *self
    }
}