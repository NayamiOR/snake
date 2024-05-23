use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn, JoinHandle};

use crossterm::event::{ read, Event, KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy)]
pub enum KeyInput {
    Up,
    Down,
    Left,
    Right,
    Quit,
}

pub struct InputDetector {
    pub key: Arc<Mutex<KeyInput>>,
}

impl Default for InputDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl InputDetector {
    pub fn new() -> InputDetector {
        InputDetector {
            key: Arc::new(Mutex::new(KeyInput::Right)),
        }
    }

    pub fn start(&mut self) -> JoinHandle<()> {
        let key = Arc::clone(&self.key);
        let action = move || {
            loop {
                if let Ok(Event::Key(key_event)) = read()
                {
                    match key_event.code {
                        KeyCode::Char('w') | KeyCode::Up => {
                            *key.lock().unwrap() = KeyInput::Up;
                        }
                        KeyCode::Char('s') | KeyCode::Down => {
                            *key.lock().unwrap() = KeyInput::Down;
                        }
                        KeyCode::Char('a') | KeyCode::Left => {
                            *key.lock().unwrap() = KeyInput::Left;
                        }
                        KeyCode::Char('d') | KeyCode::Right => {
                            *key.lock().unwrap() = KeyInput::Right;
                        }
                        KeyCode::Char('q') => {
                            *key.lock().unwrap() = KeyInput::Quit;
                        }
                        _ => {}
                    }
                }
                sleep(std::time::Duration::from_millis(100));
            }
        };
        spawn(action)
    }
}
