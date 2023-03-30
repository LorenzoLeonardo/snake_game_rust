use std::io::Stdout;

use crossterm::{style::Print, ExecutableCommand};

use crate::position::Coordinates;

pub fn draw_board(
    stdout: &mut Stdout,
    upper_left: &Coordinates,
    bottom_right: &Coordinates,
) -> Result<(), Box<dyn std::error::Error>> {
    stdout
        .execute(crossterm::cursor::MoveTo(upper_left.x, upper_left.y))?
        .execute(Print("╔"))?;

    stdout
        .execute(crossterm::cursor::MoveTo(bottom_right.x, upper_left.y))?
        .execute(Print("╗"))?;

    stdout
        .execute(crossterm::cursor::MoveTo(upper_left.x, bottom_right.y))?
        .execute(Print("╚"))?;

    stdout
        .execute(crossterm::cursor::MoveTo(bottom_right.x, bottom_right.y))?
        .execute(Print("╝"))?;

    for y in (upper_left.y + 1)..bottom_right.y {
        stdout
            .execute(crossterm::cursor::MoveTo(upper_left.x, y))?
            .execute(Print("║"))?;

        stdout
            .execute(crossterm::cursor::MoveTo(bottom_right.x, y))?
            .execute(Print("║"))?;
    }

    for x in (upper_left.x + 1)..bottom_right.x {
        stdout
            .execute(crossterm::cursor::MoveTo(x, upper_left.y))?
            .execute(Print("═"))?;

        stdout
            .execute(crossterm::cursor::MoveTo(x, bottom_right.y))?
            .execute(Print("═"))?;
    }

    let label = "Lorenzo Leonardo's Snake Game Cross-Platform Terminal Using Rust-Lang (c) 2023"
        .to_string();

    stdout
        .execute(crossterm::cursor::MoveTo(
            (bottom_right.x / 2) - (label.len() as u16 / 2),
            bottom_right.y + 1,
        ))?
        .execute(Print(label))?;
    Ok(())
}
