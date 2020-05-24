use std::time::Duration;

mod game;
mod texture_loader;

pub fn main() {
    let mut game = game::Game::new();
    'running: loop {
        game.event();
        game.update();
        game.render();
        if !game.running {
            break 'running;
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }
}