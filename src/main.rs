use ggez::{
    Context,
    ContextBuilder,
    GameResult,
    timer,
    event::{
        self,
        EventHandler
    },
    graphics::{
        self,
        Rect,
    }
};

use std::time::Duration;

#[macro_use] extern crate log;
extern crate env_logger;

enum Scene {
    MainMenu,
    Game,
}

struct GameState {
    dt: Duration, //delta time
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> GameState {
        GameState {
            dt: Duration::new(0, 0), 
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(_ctx);

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

    let (mut ctx, mut event_loop) = ContextBuilder::new("Victrix", "Ever")
        .build()
        .expect("Failed to create ggez context");
    
    graphics::set_screen_coordinates(&mut ctx, Rect::new(0.0, 0.0, 1.0, 1.0)).unwrap();

    let mut game = GameState::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Game exiting"),
        Err(e) => println!("Error: {}", e)
    }
}
