mod board;
mod food;
mod game;
mod keyboard_events;
mod position;
mod snake;

// Standard libraries
use std::io::stdout;

// 3rd party crates
use crossterm::cursor;
use crossterm::terminal;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::ExecutableCommand;
use keyboard_events::start_listening_keyboard_input;
use tokio::sync::mpsc::unbounded_channel;

// My crates
use board::draw_board;
use game::SnakeGame;
use position::Coordinates;
use snake::SnakeDirection;

// We need 2 threads for this.
// Crossterm read is blocking and the other thread
// is for the main game loop to run
#[tokio::main(worker_threads = 2)]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Need to have this for this to work on Linux environment
    enable_raw_mode()?;

    // Create a channel to send key events from the keyboard listener to the main game
    let (tx, rx) = unbounded_channel();
    let mut stdout = stdout();

    // Clear the terminal and hide the cursor before starting the game
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(cursor::Hide)?
        .execute(cursor::EnableBlinking)?;

    // Initialize the board size
    let upper_left = Coordinates::new(1, 3);
    let bottom_right = Coordinates::new(120, 37);
    draw_board(&mut stdout, &upper_left, &bottom_right)?;

    // Initialize the snake game
    let mut main_game = SnakeGame::new(upper_left, bottom_right, SnakeDirection::Right, rx);
    // Start keyboard listener
    start_listening_keyboard_input(tx);
    // Start running the game
    main_game.run(&mut stdout).await?;

    // Clear the terminal and show the cursor back before exiting the game
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(cursor::Show)?;

    // Need to have this for this to work on Linux environment
    disable_raw_mode()?;
    Ok(())
}
