/* Created by: Lorenzo Leonardo
 * Email: enzotechcomputersolutions@gmail.com
 * Date : September 15, 2022
 */
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