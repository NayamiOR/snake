use crate::render::Renderer;
use crossterm::event::{self, read, Event, KeyCode, KeyEvent, KeyModifiers};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct GameManager {
    pub render: Renderer,
    size: u32,
    grid: Rc<RefCell<Vec<Vec<char>>>>,
    snake: Snake,
    state: GameState,
    round_period: u64,
}

impl GameManager {
    pub fn new_with_size(size: u32) -> GameManager {
        let grid: Rc<RefCell<Vec<Vec<char>>>> =
            Rc::new(RefCell::new(vec![vec!['.'; size as usize]; size as usize]));
        for i in 0..size as usize {
            grid.borrow_mut()[i][0] = '#';
            grid.borrow_mut()[0][i] = '#';
            grid.borrow_mut()[size as usize - 1][i] = '#';
            grid.borrow_mut()[i][size as usize - 1] = '#';
        }
        GameManager {
            state: GameState::UnStarted,
            size: size as u32,
            grid: Rc::clone(&grid),
            render: Renderer::new(Rc::clone(&grid)),
            snake: Snake{
                location: (size / 2, size / 2),
                direction: Direction::Right,
                length: 1,
                body: VecDeque::new(),
            },
            round_period: 500,
        }
    }
    pub fn new() -> GameManager {
        GameManager::new_with_size(1)
    }

    pub fn set_round_period(&mut self, period: u64) {
        self.round_period = period;
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }

    pub fn set_state(&mut self, state: GameState) {
        self.state = state;
    }

    pub fn get_round_period(&self) -> u64 {
        self.round_period
    }

    pub fn update(&mut self) {
        for (i, v) in self.snake.get_body().iter().enumerate() {
            if i == 0 {
                self.grid.borrow_mut()[v.1 as usize][v.0 as usize] = 'O';
            } else {
                self.grid.borrow_mut()[v.1 as usize][v.0 as usize] = 'o';
            }
        }
        self.render.update();
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum GameState {
    UnStarted,
    Running,
    GameOver,
}

#[derive(Debug, Clone, Copy)]
enum KeyInput {
    Up,
    Down,
    Left,
    Right,
    Quit,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    location: (u32, u32),
    direction: Direction,
    length: u32,
    body: VecDeque<(u32, u32)>,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            location: (0, 0),
            direction: Direction::Right,
            length: 1,
            body: VecDeque::new(),
        }
    }

    fn push_body(&mut self) {
        self.body.push_back(self.location);
        if self.body.len() as u32 > self.length {
            self.body.pop_front();
        }
    }

    fn get_body(&self) -> Vec<(u32, u32)> {
        self.body.iter().map(|&x| x).collect()
    }

    fn move_direction(&mut self, direction: Direction) {
        match direction {
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
        self.length += 1;
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

trait Location {
    fn get_location(&self) -> (u32, u32);
    fn set_location(&mut self, x: u32, y: u32);
}
