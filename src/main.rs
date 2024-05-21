pub mod game;
pub mod render;

use game::GameManager;

fn main() {
    let mut gm = GameManager::new_with_size(10);
    gm.render.update();

    gm.set_state(game::GameState::Running);
    println!("Game started!");
    while let game::GameState::Running = gm.get_state() {
        gm.update();
        std::thread::sleep(std::time::Duration::from_millis(gm.get_round_period()));
    }
}
