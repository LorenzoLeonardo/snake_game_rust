

// A struct with two fields


pub fn create_food() {
    println!("create snake food");
}


pub fn is_bonus_food(lhs: u32, rhs: u32) -> bool {
    // Corner case, early return
    if rhs == 0 {
        return false;
    }

    // This is an expression, the `return` keyword is not necessary here
    lhs % rhs == 0
}