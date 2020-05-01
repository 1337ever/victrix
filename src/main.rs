use raylib::prelude::*;
use std::ffi::CString;

mod history;
mod game;

use game::*;

fn main() {
    // let mut game = Game {
    //     config: Config::new("resources/config.ron"),
    //     scene: Scene {},
    //     date: 0,
    // };

    //create a new game with config
    let mut game = Game::new("resources/config.ron");

    //Construt window
    let (mut rl, thread) = raylib::init()
        .size(game.config.size[0], game.config.size[1])
        .title(&format!("Victrix v{}", game.config.version))
        .build();

    //lock FPS
    rl.set_target_fps(game.config.fps_target);

    while !rl.window_should_close() {
        game.update(&rl);
        game.draw(&mut rl, &thread);
    }
}
