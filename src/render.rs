use std::rc::Rc;
use std::cell::RefCell;


pub struct renderer {
    grid: Rc<RefCell<Vec<Vec<char>>>>,
}

impl renderer {
    pub fn new(grid:Rc<RefCell<Vec<Vec<char>>>>) -> renderer {
        renderer{
            grid: grid,
        }
    }
    pub fn render(&self) {
        let grid = self.grid.borrow();
        for row in grid.iter() {
            for cell in row.iter() {
                print!("{} ", cell);
            }
            println!();
        }
    }
}