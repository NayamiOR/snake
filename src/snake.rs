use std::collections::VecDeque;
use crate::direction::Direction;
use crate::location::Location;

pub struct Snake {
    location: (u32, u32),
    direction: Direction,
    length: u32,
    body: VecDeque<(u32, u32)>,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            location: (1, 1),
            direction: Direction::Right,
            length: 1,
            body: VecDeque::new(),
        }
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

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => {
                self.location.1 -= 1;
            }
            Direction::Down => {
                self.location.1 += 1;
            }
            Direction::Left => {
                self.location.0 -= 1;
            }
            Direction::Right => {
                self.location.0 += 1;
            }
        }
        self.push_body();
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