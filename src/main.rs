#![feature(vec_remove_item)]

use ggez::{
    conf,
    event::{self, EventHandler},
    filesystem,
    graphics::{self, Rect, DrawParam},
    timer, Context, ContextBuilder, GameResult,
};

use gfx_glyph::{GlyphBrushBuilder, Section};

use std::{
    env,
    io::{Read, Write},
    path, str,
    time::Duration,
    collections::BTreeMap,
};

#[macro_use]
extern crate log;
extern crate env_logger;

use log::Level::{Debug, Info};

mod ui;

use ui::*;

enum Scene {
    Loading,
    MainMenu,
    Game,
}

struct MainState {
    dt: Duration, //delta time
    fps: f64,
    debug: bool,
    uielements: BTreeMap<&'static str, Box::<dyn UI>>,
    scene: Scene,
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> MainState {
        MainState {
            dt: Duration::new(0, 0),
            fps: 0f64,
            debug: log_enabled!(Debug) || log_enabled!(Info),
            scene: Scene::MainMenu,
            uielements: BTreeMap::new(),
        }
    }
    pub fn init(&mut self) {
        let button = Box::new(Button::new(10.0, 10.0, "test", 100.0, 50.0, 0x5c5c5cff));
        self.uielements.insert("1_button", button);
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(_ctx);
        self.fps = timer::fps(_ctx);

        let fpscounter = Box::new(Label::new(500.0, 10.0, &self.fps.to_string(), 0xFFFFFFFF));
        self.uielements.insert("0_fpscounter", fpscounter);


        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        graphics::clear(_ctx, graphics::BLACK);
        info!("dt: {}ms", self.dt.subsec_millis());

        for (_key, elm) in &self.uielements {
            elm.draw(_ctx)?;
        }

        graphics::draw_queued_text(
            _ctx,
            DrawParam::default(),
            None,
            graphics::FilterMode::Linear,
        )?;

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

    let (mut ctx, mut event_loop) = cb.build().expect("Failed to build context.");

    let mut game = MainState::new(&mut ctx);
    game.init();

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Game exiting"),
        Err(e) => println!("Error: {}", e),
    }
}
