#[macro_use]
extern crate conrod_core;

use std::fs;

use ron::de::from_str;
use serde::Deserialize;

use conrod_core::{widget, Colorable, Positionable, Widget};
use glium::Surface;

mod history;
mod support;

//Struct for config data like game settings
#[derive(Debug, Deserialize)]
pub struct Config {
    pub size: [f64; 2],
    pub fullscreen: bool,
    pub vsync: bool,
    pub antialiasing: u16,
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

    //Create window
    let size = glium::glutin::dpi::LogicalSize::new(game.config.size[0], game.config.size[1]);
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Victrix")
        .with_dimensions(size);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(game.config.vsync)
        .with_multisampling(game.config.antialiasing);
    let display =
        glium::Display::new(window, context, &events_loop).expect("Failed to create display");
    let display = support::GliumDisplayWinitWrapper(display);

    //Construct UI
    let mut ui =
        conrod_core::UiBuilder::new([game.config.size[0] as f64, game.config.size[1] as f64])
            .build();

    //Generate widget IDs
    let ids = Ids::new(ui.widget_id_generator());

    //Add a font to the font::Map
    let font_path = "resources/OpenSans-Regular.ttf";
    ui.fonts
        .insert_from_file(font_path)
        .expect("Failed to insert font");

    //convert conrod primitives into commands for drawing to glium Surface
    let mut renderer = conrod_glium::Renderer::new(&display.0).expect("Failed to create Renderer");

    let image_map = conrod_core::image::Map::<glium::texture::Texture2d>::new();

    let mut list = vec![true; 16];

    // Poll events from the window.
    let mut event_loop = support::EventLoop::new();
    'main: loop {
        // Handle all events.
        for event in event_loop.next(&mut events_loop) {
            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = support::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::WindowEvent::CloseRequested
                    | glium::glutin::WindowEvent::KeyboardInput {
                        input:
                            glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => break 'main,
                    _ => (),
                },
                _ => (),
            }
        }

        set_ui(ui.set_widgets(), &mut list, &ids);

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display.0, primitives, &image_map);
            let mut target = display.0.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display.0, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

// Declare the `WidgetId`s and instantiate the widgets.
fn set_ui(ref mut ui: conrod_core::UiCell, list: &mut [bool], ids: &Ids) {
    use conrod_core::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};

    widget::Canvas::new()
        .color(conrod_core::color::DARK_CHARCOAL)
        .set(ids.canvas, ui);

    let (mut items, scrollbar) = widget::List::flow_down(list.len())
        .item_size(50.0)
        .scrollbar_on_top()
        .middle_of(ids.canvas)
        .wh_of(ids.canvas)
        .set(ids.list, ui);

    while let Some(item) = items.next(ui) {
        let i = item.i;
        let label = format!("item {}: {}", i, list[i]);
        let toggle = widget::Toggle::new(list[i])
            .label(&label)
            .label_color(conrod_core::color::WHITE)
            .color(conrod_core::color::LIGHT_BLUE);
        for v in item.set(toggle, ui) {
            list[i] = v;
        }
    }

    if let Some(s) = scrollbar {
        s.set(ui)
    }
}
