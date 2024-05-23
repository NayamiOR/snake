use crate::input_detector::{InputDetector};
use crate::render::Renderer;
use crate::snake::Snake;
use std::cell::RefCell;
use std::rc::Rc;
use crate::location::Location;
use rand;

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
    input_detector: InputDetector,
    current_beam: Option<(u32, u32)>,
}

impl Default for GameManager {
    fn default() -> Self {
        Self::new()
    }
}

impl GameManager {
    pub fn new() -> GameManager {
        let size = 3;
        let grid: Rc<RefCell<Vec<Vec<char>>>> =
            Rc::new(RefCell::new(vec![vec![' '; size as usize]; size as usize]));
        for i in 0..size as usize {
            grid.borrow_mut()[i][0] = '#';
            grid.borrow_mut()[0][i] = '#';
            grid.borrow_mut()[size as usize - 1][i] = '#';
            grid.borrow_mut()[i][size as usize - 1] = '#';
        }
        let mut snake = Snake::new();
        snake.set_location(size / 2, size / 2);
        let ditector = InputDetector::new();
        snake.set_direction_source(ditector.key.clone());
        GameManager {
            size,
            state: GameState::UnStarted,
            grid: Rc::clone(&grid),
            render: Renderer::new(Rc::clone(&grid)),
            snake,
            round_period: 500,
            input_detector: ditector,
            current_beam: None,
        }
    }

    pub fn start_game(&mut self) {
        self.set_state(GameState::Running);
        self.input_detector.start();
    }

    pub fn new_with_size(size: u32) -> GameManager {
        let mut gm = GameManager::new();
        gm.size = size;
        gm.make_grid();
        gm
    }

    fn wash_grid(&mut self) {
        let mut grid = self.grid.borrow_mut();
        let size = self.size;
        for i in 0..size as usize {
            for j in 0..size as usize {
                grid[i][j] = ' ';
            }
        }
        for i in 0..size as usize {
            grid[i][0] = '#';
            grid[0][i] = '#';
            grid[size as usize - 1][i] = '#';
            grid[i][size as usize - 1] = '#';
        }
        if let Some((x, y)) = self.current_beam {
            grid[y as usize][x as usize] = '*';
        }
    }

    fn make_grid(&mut self) {
        let size = self.size;
        let mut grid = self.grid.borrow_mut();
        *grid = vec![vec![' '; size as usize]; size as usize];
    }
    pub fn get_size(&self) -> u32 {
        self.size
    }


    pub fn get_state(&self) -> GameState {
        self.state
    }

    pub fn set_state(&mut self, state: GameState) {
        self.state = state;
    }

    pub fn set_round_period(&mut self, period: u64) {
        self.round_period = period;
    }

    pub fn get_round_period(&self) -> u64 {
        self.round_period
    }


    pub fn update(&mut self) {
        // 判断会不会吃到，下一步会吃到的话要提前处理
        if self.pre_bean() {
            self.snake.grow();
            self.current_beam = None;
        }
        self.snake.move_forward();

        // 判断是否碰撞
        if self.check_collision() {
            self.game_over();
        }

        // 开始渲染
        self.wash_grid();

        for (i, v) in self.snake.get_body().iter().enumerate() {
            self.grid.borrow_mut()[v.1 as usize][v.0 as usize] = 'o';
        }
        self.grid.borrow_mut()[self.snake.get_location().1 as usize][self.snake.get_location().0 as usize] = '@';

        // 判断是否吃到豆子
        if self.current_beam.is_none() {
            self.current_beam = Some(self.generate_bean());
        }

        self.render.update();
    }

    fn game_over(&mut self) {
        self.set_state(GameState::GameOver);
    }

    fn check_collision(&self) -> bool {
        let head = self.snake.get_location();
        let body = self.snake.get_body();
        for b in body.iter() {
            if head == *b {
                return true;
            }
        }
        if let (x, y) = head {
            if x == 0 || y == 0 || x == self.size - 1 || y == self.size - 1 {
                return true;
            }
        }
        false
    }

    // fn check_bean(&self) -> bool {
    //     let head = self.snake.get_location();
    //     if let Some((x, y)) = self.current_beam {
    //         if head == (x, y) {
    //             return true;
    //         }
    //     }
    //     false
    // }

    fn pre_bean(&self)->bool{
        let head=self.snake.next_location();
        if let Some((x,y))=self.current_beam{
            if head==(x,y){
                return true;
            }
        }
        false
    }

    fn generate_bean(&self) -> (u32, u32) {
        let mut x = 0;
        let mut y = 0;
        loop {
            x = rand::random::<u32>() % self.size;
            y = rand::random::<u32>() % self.size;
            if self.grid.borrow()[y as usize][x as usize] == ' ' {
                break;
            }
        }
        (x, y)
    }
}
