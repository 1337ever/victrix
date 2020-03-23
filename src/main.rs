#[macro_use]
extern crate conrod_core;

use std::fs;

use ron::de::from_str;
use serde::Deserialize;

use piston_window::texture::UpdateTexture;
use piston_window::OpenGL;
use piston_window::{Flip, G2d, G2dTexture, Texture, TextureSettings};
use piston_window::{PistonWindow, UpdateEvent, Window, WindowSettings};

mod history;

//Struct for config data like game settings
#[derive(Debug, Deserialize)]
pub struct Config {
    pub size: [f64; 2],
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
widget_ids!(struct Ids { canvas, list });

fn main() {
    let mut game = Game {
        config: Config::new("resources/config.ron"),
        scene: Scene {},
        date: 0,
    };

    //Construt window
    let mut window: PistonWindow = WindowSettings::new("Victrix", game.config.size)
        .opengl(OpenGL::V3_2)
        .samples(game.config.antialiasing)
        .exit_on_esc(true)
        .vsync(game.config.vsync)
        .size(game.config.size.into())
        .fullscreen(game.config.fullscreen)
        .build()
        .expect("Failed to construct window.");

    //Construct Ui
    let mut ui = conrod_core::UiBuilder::new(game.config.size).build();

    //Add font to Ui's font::Map
    ui.fonts
        .insert_from_file("resources/OpenSans-Regular.ttf")
        .expect("Failed to insert font from file.");

    //Create a texture to cache text
    let mut text_vertex_data = Vec::new();
    let (mut glyph_cache, mut text_texture_cache) = {
        const SCALE_TOLERANCE: f32 = 0.1;
        const POSITION_TOLERANCE: f32 = 0.1;
        let cache = conrod_core::text::GlyphCache::builder()
            .dimensions(game.config.size[0] as u32, game.config.size[1] as u32)
            .scale_tolerance(SCALE_TOLERANCE)
            .position_tolerance(POSITION_TOLERANCE)
            .build();
        let buffer_len = game.config.size[0] as usize * game.config.size[1] as usize;
        let init = vec![128; buffer_len];
        let settings = TextureSettings::new();
        let factory = &mut window.factory;
        let texture = G2dTexture::from_memory_alpha(
            factory,
            &init,
            game.config.size[0] as u32,
            game.config.size[1] as u32,
            &settings,
        )
        .expect("Failed to create G2dTexture for text caching.");
        (cache, texture)
    };

    //Instantiate list of widget IDs
    //let ids = conrod_example_shared::Ids::new(ui.widget_id_generator());

    let mut image_map = conrod_core::image::Map::new();

    while let Some(event) = window.next() {
        //Convert event to conrod event
        let size = window.size();
        let (win_w, win_h) = (
            size.width as conrod_core::Scalar,
            size.height as conrod_core::Scalar,
        );
        if let Some(e) = conrod_piston::event::convert(event.clone(), win_w, win_h) {
            ui.handle_event(e);
        }

        event.update(|_| {
            let mut ui = ui.set_widgets();
            //conrod_example_shared::gui(&mut ui, &ids, &mut app);
        });

        window.draw_2d(&event, |context, graphics| {
            if let Some(primitives) = ui.draw_if_changed() {
                //Cache glyphs to texture cache
                let cache_queued_glyphs = |graphics: &mut G2d,
                                           cache: &mut G2dTexture,
                                           rect: conrod_core::text::rt::Rect<u32>,
                                           data: &[u8]| {
                    let offset = [rect.min.x, rect.min.y];
                    let size = [rect.width(), rect.height()];
                    let format = piston_window::texture::Format::Rgba8;
                    let encoder = &mut graphics.encoder;
                    text_vertex_data.clear();
                    text_vertex_data.extend(data.iter().flat_map(|&b| vec![255, 255, 255, b]));
                    UpdateTexture::update(
                        cache,
                        encoder,
                        format,
                        &text_vertex_data[..],
                        offset,
                        size,
                    )
                    .expect("Failed to update texture cache")
                };

                fn texture_from_image<T>(img: &T) -> &T {
                    img
                }

                //Draw conrod primitives
                conrod_piston::draw::primitives(
                    primitives,
                    context,
                    graphics,
                    &mut text_texture_cache,
                    &mut glyph_cache,
                    &image_map,
                    cache_queued_glyphs,
                    texture_from_image,
                );
            }
        });
    }
}
