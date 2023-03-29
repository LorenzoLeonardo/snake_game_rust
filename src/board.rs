use std::io::Stdout;

use crossterm::{style::Print, ExecutableCommand};

use crate::position::Coordinates;

pub fn draw_board(
    stdout: &mut Stdout,
    screen_size: Coordinates,
) -> Result<(), Box<dyn std::error::Error>> {
    stdout
        .execute(crossterm::cursor::MoveTo(1, 1))?
        .execute(Print("╔"))?;

    stdout
        .execute(crossterm::cursor::MoveTo(screen_size.x, 1))?
        .execute(Print("╗"))?;

    stdout
        .execute(crossterm::cursor::MoveTo(1, screen_size.y))?
        .execute(Print("╚"))?;

    stdout
        .execute(crossterm::cursor::MoveTo(screen_size.x, screen_size.y))?
        .execute(Print("╝"))?;

    for y in 2..screen_size.y {
        stdout
            .execute(crossterm::cursor::MoveTo(1, y))?
            .execute(Print("║"))?;

        stdout
            .execute(crossterm::cursor::MoveTo(screen_size.x, y))?
            .execute(Print("║"))?;
    }

    for x in 2..screen_size.x {
        stdout
            .execute(crossterm::cursor::MoveTo(x, 1))?
            .execute(Print("═"))?;

        stdout
            .execute(crossterm::cursor::MoveTo(x, screen_size.y))?
            .execute(Print("═"))?;
    }

    let label = "Lorenzo Leonardo's Snake Game Cross-Platform Terminal Using Rust-Lang (c) 2023"
        .to_string();

    stdout
        .execute(crossterm::cursor::MoveTo(
            (screen_size.x / 2) - (label.len() as u16 / 2),
            screen_size.y + 1,
        ))?
        .execute(Print(label))?;
    Ok(())
}
