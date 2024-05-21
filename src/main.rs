pub mod render;
pub mod snake;
pub mod game;

use game::GameManager;
fn main() {
    let gm=GameManager::new_with_size(10);
    gm.render.render();
}
