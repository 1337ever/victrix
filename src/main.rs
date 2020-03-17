use std::fs;

use ron::de::from_str;
use serde::Deserialize;

use piston_window::*;

//Struct for config data like game settings
#[derive(Debug, Deserialize)]
pub struct Config {
    pub window_size: (u16, u16),
    pub fullscreen: bool,
    pub vsync: bool,

    pub volume: f32,     //float between 0 and 1
    pub mouse_sens: f32, //float between 0 and 1
}

impl Config {
    pub fn new(config_path: &str) -> Config {
        let configstring = fs::read_to_string(config_path)
            .expect(&format!("Failed to read config at {}", config_path));

        let config: Config = match from_str(&configstring) {
            Ok(v) => v,
            Err(e) => {
                println!("Failed to load config at {}", e);
                std::process::exit(1);
            }
        };
        return config;
    }
}

pub struct Scene {

}

pub struct Game {
    config: Config,
    scene: Scene,
}

fn main() {
    let mut game = Game {
        config: Config::new("resources/config.ron"),
        scene: Scene {},
    };

    let mut window: PistonWindow = WindowSettings::new("Victrix", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle(
                [1.0, 0.0, 0.0, 1.0], //red
                [0.0, 0.0, 100.0, 100.0],
                context.transform,
                graphics,
            );
        });
    }
}
