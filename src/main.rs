use std::fs;

use ron::de::from_str;
use serde::Deserialize;

use piston_window::*;

mod history;

//Struct for config data like game settings
#[derive(Debug, Deserialize)]
pub struct Config {
    pub window_size: [u32; 2],
    pub fullscreen: bool,
    pub vsync: bool,

    pub volume: f32,     //float between 0 and 1
    pub mouse_sens: f32, //float between 0 and 1

    pub version: f32,
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

//struct of all game data which should be saveable
pub struct Save {}

pub struct Scene {}

pub struct Game {
    config: Config,
    scene: Scene,
    date: u8,
}

fn main() {
    let mut game = Game {
        config: Config::new("resources/config.ron"),
        scene: Scene {},
        date: 0,
    };

    let mut window: PistonWindow = WindowSettings::new("Victrix", game.config.window_size)
        .exit_on_esc(true)
        .vsync(game.config.vsync)
        .fullscreen(game.config.fullscreen)
        .build()
        .expect("Failed to create window with given configuration.");
}
