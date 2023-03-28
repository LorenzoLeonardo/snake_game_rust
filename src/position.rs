/* Created by: Lorenzo Leonardo
 * Email: enzotechcomputersolutions@gmail.com
 * Date : September 15, 2022
 */
#[derive(PartialEq, Clone, Copy)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

impl Coordinates {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}
