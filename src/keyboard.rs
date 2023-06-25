// Standard libraries
use std::time::Duration;

// 3rd party crates
use crossterm::event::{poll, Event};
use crossterm::event::{read, KeyCode};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;

use crate::engine::SnakeGameState;

pub struct KeyboardListener {
    tx_key_event: UnboundedSender<KeyCode>,
    rx_game_state: UnboundedReceiver<SnakeGameState>,
}

impl KeyboardListener {
    pub fn new(
        tx_key_event: UnboundedSender<KeyCode>,
        rx_game_state: UnboundedReceiver<SnakeGameState>,
    ) -> Self {
        Self {
            tx_key_event,
            rx_game_state,
        }
    }
    pub async fn run(mut self) -> JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                match self.rx_game_state.try_recv() {
                    Ok(state) => match state {
                        SnakeGameState::SnakeDied | SnakeGameState::Quit => break,
                    },
                    Err(_err) => {
                        if poll(Duration::from_millis(100)).unwrap() {
                            if let Event::Key(key_event) = read().unwrap() {
                                match key_event.code {
                                    KeyCode::Down => {
                                        if let Err(e) = self.tx_key_event.send(KeyCode::Down) {
                                            eprintln!("Error: {:?}", e);
                                        }
                                    }
                                    KeyCode::Up => {
                                        if let Err(e) = self.tx_key_event.send(KeyCode::Up) {
                                            eprintln!("Error: {:?}", e);
                                        }
                                    }
                                    KeyCode::Left => {
                                        if let Err(e) = self.tx_key_event.send(KeyCode::Left) {
                                            eprintln!("Error: {:?}", e);
                                        }
                                    }
                                    KeyCode::Right => {
                                        if let Err(e) = self.tx_key_event.send(KeyCode::Right) {
                                            eprintln!("Error: {:?}", e);
                                        }
                                    }
                                    KeyCode::Esc => {
                                        if let Err(e) = self.tx_key_event.send(KeyCode::Esc) {
                                            eprintln!("Error: {:?}", e);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}
