use crate::render::renderer;
use std::cell::RefCell;
use std::rc::Rc;

pub struct GameManager {
    size: u32,
    grid: Rc<RefCell<Vec<Vec<char>>>>,
    pub render: renderer,
}

impl GameManager {
    pub fn new_with_size(size:u32)->GameManager{
        let grid: Rc<RefCell<Vec<Vec<char>>>> = Rc::new(RefCell::new(vec![vec![' '; size as usize]; size as usize]));
        for i in 0..size as usize {
            grid.borrow_mut()[i][0] = '#';
            grid.borrow_mut()[0][i] = '#';
            grid.borrow_mut()[size as usize - 1][i] = '#';
            grid.borrow_mut()[i][size as usize - 1] = '#';
        }
        GameManager{
            size: size as u32,
            grid: Rc::clone(&grid),
            render: renderer::new(Rc::clone(&grid)),
        }
    }
    pub fn new() -> GameManager {
        GameManager::new_with_size(1)
    }
}
