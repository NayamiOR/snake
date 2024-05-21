use std::cell::RefCell;
use std::rc::Rc;

pub struct Renderer {
    grid: Rc<RefCell<Vec<Vec<char>>>>,
}

impl Renderer {
    pub fn new(grid: Rc<RefCell<Vec<Vec<char>>>>) -> Renderer {
        Renderer { grid: grid }
    }
    pub fn update(&self) {
        print!("\x1b[2J");
        print!("\x1b[H");
        let grid: std::cell::Ref<Vec<Vec<char>>> = self.grid.borrow();
        for row in grid.iter() {
            for cell in row.iter() {
                print!("{} ", cell);
            }
            println!();
        }
    }
}
