use crossterm::event::Event;
use crossterm::event::{read, KeyCode};
use tokio::sync::mpsc::UnboundedSender;

pub fn start_listening_keyboard_input(tx: UnboundedSender<KeyCode>) {
    tokio::spawn(async move {
        loop {
            if let Event::Key(key_event) = read().unwrap() {
                match key_event.code {
                    KeyCode::Down => {
                        tx.send(KeyCode::Down).unwrap();
                    }
                    KeyCode::Up => {
                        tx.send(KeyCode::Up).unwrap();
                    }
                    KeyCode::Left => {
                        tx.send(KeyCode::Left).unwrap();
                    }
                    KeyCode::Right => {
                        tx.send(KeyCode::Right).unwrap();
                    }
                    KeyCode::Esc => {
                        tx.send(KeyCode::Esc).unwrap();
                        break;
                    }
                    _ => {}
                }
            }
        }
    });
}
