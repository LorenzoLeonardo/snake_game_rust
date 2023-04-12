mod draw;
mod engine;
mod food;
mod keyboard;
mod position;
mod snake;

// 3rd party crates
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use keyboard::KeyboardListener;
use tokio::sync::mpsc::unbounded_channel;
// My crates
use engine::GameEngine;
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
    let (tx_key_event, rx_key_event) = unbounded_channel();
    let (tx_snake_died, rx_snake_died) = unbounded_channel();
    // Initialize the board size
    let upper_left = Coordinates::new(1, 3);
    let bottom_right = Coordinates::new(120, 37);
    // Initialize the keyboard listener
    let keyboard_listener = KeyboardListener::new(tx_key_event, rx_snake_died);
    // Initialize the snake game
    let mut main_game = GameEngine::new(
        upper_left,
        bottom_right,
        SnakeDirection::Right,
        rx_key_event,
        tx_snake_died,
    );
    // Run keyboard listener
    let keyboard_handler = keyboard_listener.run();
    // Run the game
    main_game.run().await?;
    // We make sure all threads are terminated
    let _ = tokio::join!(keyboard_handler);
    // Need to have this for this to work on Linux environment
    disable_raw_mode()?;
    Ok(())
}
