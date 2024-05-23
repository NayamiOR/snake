use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use crate::direction::Direction;
use crate::input_detector::KeyInput;
use crate::location::Location;

pub struct Snake {
    // direction: Direction,
    direction: Arc<Mutex<KeyInput>>,
    location: (u32, u32),
    length: u32,
    body: VecDeque<(u32, u32)>,
}

impl Default for Snake {
    fn default() -> Self {
        Self::new()
    }
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            location: (1, 1),
            direction: Arc::new(Mutex::new(KeyInput::Right)),
            length: 1,
            body: VecDeque::new(),
        }
    }

    pub fn set_direction_source(&mut self, direction: Arc<Mutex<KeyInput>>) {
        self.direction = direction;
    }

    pub fn push_body(&mut self) {
        self.body.push_back(self.location);
        if self.body.len() as u32 > self.length {
            self.body.pop_front();
        }
    }

    pub fn get_body(&self) -> Vec<(u32, u32)> {
        self.body.iter().map(|&x| x).collect()
    }
    
    pub fn next_location(&self) -> (u32, u32) {
        let mut next_location = self.location;
        match *self.direction.lock().unwrap() {
            KeyInput::Up => {
                next_location.1 -= 1;
            }
            KeyInput::Down => {
                next_location.1 += 1;
            }
            KeyInput::Left => {
                next_location.0 -= 1;
            }
            KeyInput::Right => {
                next_location.0 += 1;
            }
            _ => {}
        }
        
        next_location
    }

    pub fn move_forward(&mut self) {
        self.push_body();
        self.location = self.next_location();
    }
    
    pub fn grow(&mut self) {
        self.length += 1;
    }
}

impl Location for Snake {
    fn get_location(&self) -> (u32, u32) {
        self.location
    }
    fn set_location(&mut self, x: u32, y: u32) {
        self.location = (x, y);
    }
}