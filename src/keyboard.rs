// Standard libraries
use std::time::Duration;

// 3rd party crates
use crossterm::event::{poll, Event};
use crossterm::event::{read, KeyCode};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;

pub struct KeyboardListener {
    tx_key_event: UnboundedSender<KeyCode>,
    rx_snake_died: UnboundedReceiver<bool>,
}

impl KeyboardListener {
    pub fn new(
        tx_key_event: UnboundedSender<KeyCode>,
        rx_snake_died: UnboundedReceiver<bool>,
    ) -> Self {
        Self {
            tx_key_event,
            rx_snake_died,
        }
    }
    pub async fn run(mut self) -> JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                match self.rx_snake_died.try_recv() {
                    Ok(recv) => {
                        if recv {
                            break;
                        }
                    }
                    Err(_err) => {
                        if poll(Duration::from_millis(100)).unwrap() {
                            if let Event::Key(key_event) = read().unwrap() {
                                match key_event.code {
                                    KeyCode::Down => {
                                        if let Err(_e) = self.tx_key_event.send(KeyCode::Down) {}
                                    }
                                    KeyCode::Up => {
                                        if let Err(_e) = self.tx_key_event.send(KeyCode::Up) {}
                                    }
                                    KeyCode::Left => {
                                        if let Err(_e) = self.tx_key_event.send(KeyCode::Left) {}
                                    }
                                    KeyCode::Right => {
                                        if let Err(_e) = self.tx_key_event.send(KeyCode::Right) {}
                                    }
                                    KeyCode::Esc => {
                                        if let Err(_e) = self.tx_key_event.send(KeyCode::Esc) {}
                                        break;
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
