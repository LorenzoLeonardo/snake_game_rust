/* Created by: Lorenzo Leonardo
 * Email: enzotechcomputersolutions@gmail.com
 * Date : September 15, 2022
 */
#[derive(PartialEq, Clone, Copy)]
pub struct Coordinates {
    pub x: u32,
    pub y: u32,
}

impl Coordinates {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x: x, y: y }
    }
}
