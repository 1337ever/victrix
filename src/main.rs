use ggez::{
    event::{self, EventHandler},
    graphics::{self, Rect},
    timer, Context, ContextBuilder, GameResult, conf, filesystem,
};

use gfx_glyph::{Section, GlyphBrushBuilder};

use std::{
    time::Duration,
    io::{Read, Write},
    env,
    path,
    str
};

#[macro_use]
extern crate log;
extern crate env_logger;

enum Scene {
    Loading,
    MainMenu,
    Game,
}

struct GameState {
    dt: Duration, //delta time
    fps: f64,
    scene: Scene,
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> GameState {
        GameState {
            dt: Duration::new(0, 0),
            fps: 0f64,
            scene: Scene::Loading,
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(_ctx);
        self.fps = timer::fps(_ctx);

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        graphics::clear(_ctx, graphics::BLACK);
        info!("dt: {}ms", self.dt.subsec_millis());

        graphics::present(_ctx)
    }
}

fn main() {
    env_logger::init();

    let mut cb = ContextBuilder::new("Victrix", "Ever");

    //set up resource directories
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        info!("Adding path {:?}", path);
        cb = cb.add_resource_path(path); 
    }

    let (mut ctx, mut event_loop) = cb.build()
        .expect("Failed to build context.");

    graphics::set_screen_coordinates(&mut ctx, Rect::new(0.0, 0.0, 1.0, 1.0)).unwrap();

    let mut game = GameState::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Game exiting"),
        Err(e) => println!("Error: {}", e),
    }
}
