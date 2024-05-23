pub mod game;
pub mod render;
pub mod snake;
pub mod direction;
pub mod location;
pub mod input_detector;

use game::GameManager;
use input_detector::InputDetector;

fn main() {
    let size=10;
    let mut gm = GameManager::new_with_size(size);
    gm.set_round_period(500);
    gm.start_game();
    while let game::GameState::Running = gm.get_state() {
        gm.update();
        std::thread::sleep(std::time::Duration::from_millis(gm.get_round_period()));
    }
}
