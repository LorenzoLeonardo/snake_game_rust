use std::{
    borrow::Cow,
    io::{stdout, Stdout},
};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, style::Print, terminal, ExecutableCommand};

use crate::position::Coordinates;

pub struct Draw {
    stdout: Stdout,
}

impl Draw {
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        // Need to have this for this to work on Linux environment
        enable_raw_mode()?;
        let mut stdout = stdout();
        // Clear the terminal and hide the cursor before starting the game
        stdout
            .execute(terminal::Clear(terminal::ClearType::All))?
            .execute(cursor::Hide)?
            .execute(cursor::EnableBlinking)?;

        Ok(Self { stdout })
    }

    pub fn deinit(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Clear the terminal and show the cursor back before exiting the game
        self.stdout
            .execute(crossterm::cursor::MoveTo(0, 0))?
            .execute(terminal::Clear(terminal::ClearType::All))?
            .execute(cursor::Show)?;
        // Need to have this for this to work on Linux environment
        disable_raw_mode()?;
        Ok(())
    }

    pub fn draw_snake(&mut self, snake_body: &[Coordinates], body_style: Cow<&str>) {
        let mut i = 0;
        while i < snake_body.len() {
            self.stdout
                .execute(crossterm::cursor::MoveTo(snake_body[i].x, snake_body[i].y))
                .unwrap()
                .execute(Print(&body_style))
                .unwrap();

            i += 1;
        }
    }

    pub fn draw_food(&mut self, food_position: &Coordinates, food_style: Cow<&str>) {
        self.stdout
            .execute(crossterm::cursor::MoveTo(food_position.x, food_position.y))
            .unwrap()
            .execute(Print(&food_style))
            .unwrap();
    }

    pub fn remove_snake_trail(&mut self, body_trail: &Coordinates) {
        self.stdout
            .execute(crossterm::cursor::MoveTo(body_trail.x, body_trail.y))
            .unwrap()
            .execute(Print(" "))
            .unwrap();
    }

    pub fn draw_board(
        &mut self,
        upper_left: &Coordinates,
        bottom_right: &Coordinates,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.stdout
            .execute(crossterm::cursor::MoveTo(upper_left.x, 1))?
            .execute(Print("╔"))?;

        self.stdout
            .execute(crossterm::cursor::MoveTo(bottom_right.x, 1))?
            .execute(Print("╗"))?;

        self.stdout
            .execute(crossterm::cursor::MoveTo(upper_left.x, 2))?
            .execute(Print("║"))?;

        self.stdout
            .execute(crossterm::cursor::MoveTo(bottom_right.x, 2))?
            .execute(Print("║"))?;

        for x in (upper_left.x + 1)..bottom_right.x {
            self.stdout
                .execute(crossterm::cursor::MoveTo(x, 1))?
                .execute(Print("═"))?;
        }

        self.stdout
            .execute(crossterm::cursor::MoveTo(2, 2))?
            .execute(Print(
                " Quit(Esc)     Up(↑)     Down(↓)     Left(←)     Right(→)",
            ))?;

        self.stdout
            .execute(crossterm::cursor::MoveTo(upper_left.x, upper_left.y))?
            .execute(Print("╠"))?;

        self.stdout
            .execute(crossterm::cursor::MoveTo(bottom_right.x, upper_left.y))?
            .execute(Print("╣"))?;

        self.stdout
            .execute(crossterm::cursor::MoveTo(upper_left.x, bottom_right.y))?
            .execute(Print("╚"))?;

        self.stdout
            .execute(crossterm::cursor::MoveTo(bottom_right.x, bottom_right.y))?
            .execute(Print("╝"))?;

        for y in (upper_left.y + 1)..bottom_right.y {
            self.stdout
                .execute(crossterm::cursor::MoveTo(upper_left.x, y))?
                .execute(Print("║"))?;

            self.stdout
                .execute(crossterm::cursor::MoveTo(bottom_right.x, y))?
                .execute(Print("║"))?;
        }

        for x in (upper_left.x + 1)..bottom_right.x {
            self.stdout
                .execute(crossterm::cursor::MoveTo(x, upper_left.y))?
                .execute(Print("═"))?;

            self.stdout
                .execute(crossterm::cursor::MoveTo(x, bottom_right.y))?
                .execute(Print("═"))?;
        }

        let label =
            "Lorenzo Leonardo's Snake Game Cross-Platform Terminal Using Rust-Lang (c) 2023"
                .to_string();

        self.stdout
            .execute(crossterm::cursor::MoveTo(
                (bottom_right.x / 2) - (label.len() as u16 / 2),
                bottom_right.y + 1,
            ))?
            .execute(Print(label))?;
        Ok(())
    }
}
