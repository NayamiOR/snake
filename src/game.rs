use crate::render::Renderer;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use crate::snake::Snake;

#[derive(Debug, Clone, Copy)]
pub(crate) enum GameState {
    UnStarted,
    Running,
    GameOver,
}

pub struct GameManager {
    pub render: Renderer,
    size: u32,
    grid: Rc<RefCell<Vec<Vec<char>>>>,
    snake: Snake,
    state: GameState,
    round_period: u64,
}

impl GameManager {
    pub fn new() -> GameManager {
        let size = 1;
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
            size,
            grid: Rc::clone(&grid),
            render: Renderer::new(Rc::clone(&grid)),
            snake: Snake::new(),
            round_period: 500,
        }
    }
    pub fn new_with_size(size: u32) -> GameManager {
        let mut gm=GameManager::new();
        gm.size=size;
        gm
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
