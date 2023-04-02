use crossterm::event::Event;
use crossterm::event::{read, KeyCode};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub fn start_listening_keyboard_input(
    tx_key_event: UnboundedSender<KeyCode>,
    mut rx_snake_died: UnboundedReceiver<bool>,
) {
    tokio::spawn(async move {
        loop {
            match rx_snake_died.try_recv() {
                Ok(recv) => {
                    if recv {
                        break;
                    }
                }
                Err(_err) => {
                    if let Event::Key(key_event) = read().unwrap() {
                        match key_event.code {
                            KeyCode::Down => if let Err(_e) = tx_key_event.send(KeyCode::Down) {},
                            KeyCode::Up => if let Err(_e) = tx_key_event.send(KeyCode::Up) {},
                            KeyCode::Left => if let Err(_e) = tx_key_event.send(KeyCode::Left) {},
                            KeyCode::Right => if let Err(_e) = tx_key_event.send(KeyCode::Right) {},
                            KeyCode::Esc => {
                                if let Err(_e) = tx_key_event.send(KeyCode::Esc) {}
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    });
}
