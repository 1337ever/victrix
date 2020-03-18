use std::fs;

use ron::de::from_str;
use serde::Deserialize;

extern crate conrod_core;
extern crate conrod_piston;
extern crate piston_window;

use piston_window::texture::UpdateTexture;
use piston_window::OpenGL;
use piston_window::{Flip, G2d, G2dTexture, Texture, TextureSettings};
use piston_window::{PistonWindow, UpdateEvent, Window, WindowSettings};

mod history;

//Struct for config data like game settings
#[derive(Debug, Deserialize)]
pub struct Config {
    pub window_size: [u32; 2],
    pub fullscreen: bool,
    pub vsync: bool,
    pub antialiasing: u8,
    pub font: String,

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

    /*let theme = conrod_core::Theme {
        name: "Default theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod_core::color::DARK_CHARCOAL,
        shape_color: conrod_core::color::LIGHT_CHARCOAL,
        border_color: conrod_core::color::BLACK,
        border_width: 0.0,
        label_color: conrod_core::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: conrod_core::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    };*/

    let mut window: PistonWindow = WindowSettings::new("Victrix", game.config.window_size)
        .exit_on_esc(true)
        .samples(game.config.antialiasing)
        .vsync(game.config.vsync)
        .fullscreen(game.config.fullscreen)
        .build()
        .expect("Failed to create window with given configuration.");

    //construct UI
    let mut ui = conrod_core::UiBuilder::new([
        game.config.window_size[0] as f64,
        game.config.window_size[1] as f64,
    ])
    .build();

    //Load font
    ui.fonts
        .insert_from_file(game.config.font)
        .expect("Unable to load font.");

    //create texture to cache text
    let mut text_vertex_data: Vec<u8> = Vec::new();
    let (mut glyph_cache, mut text_texture_cache) = {
        const SCALE_TOLERANCE: f32 = 0.1;
        const POSITION_TOLERANCE: f32 = 0.1;
        let cache = conrod_core::text::GlyphCache::builder()
            .dimensions(game.config.window_size[0], game.config.window_size[1])
            .scale_tolerance(SCALE_TOLERANCE)
            .position_tolerance(POSITION_TOLERANCE)
            .build();
        let buffer_len = game.config.window_size[0] as usize * game.config.window_size[1] as usize;
        let init = vec![128; buffer_len];
        let settings = TextureSettings::new();
        let texture_context = &mut window.create_texture_context();
        let texture = G2dTexture::from_memory_alpha(
            texture_context,
            &init,
            game.config.window_size[0],
            game.config.window_size[1],
            &settings,
        )
        .unwrap();
        (cache, texture)
    };

    //let mut events = Events::new(EventSettings::new());

    while let Some(event) = window.next() {
        let size = window.size();
        let (win_w, win_h) = (
            size.width as conrod_core::Scalar,
            size.height as conrod_core::Scalar,
        );
        if let Some(e) = conrod_piston::event::convert(event.clone(), win_w, win_h) {
            ui.handle_event(e);
        }
    }
    /*while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }*/
}
