//this file contains the code for managing game objects and scenes and stuff

use raylib::prelude::*;
use std::fs;

use ron::de::from_str;
use serde::Deserialize;

//trait for all objects in the game
trait GameObject {
    fn destroy(&self);
    fn update(&mut self, rl: &RaylibHandle);
    fn draw(&self, d: &mut RaylibDrawHandle);
}

//this is just a test GameObject
struct Rect {
    rect: Rectangle,
}
impl Rect {
    fn new() -> Rect {
        Rect {
            rect: Rectangle::new(400.0, 200.0, 40.0, 40.0),
        }
    }
}
impl GameObject for Rect {
    fn destroy(&self) {
        println!("Destroyed");
    }
    fn update(&mut self, rl: &RaylibHandle) {}
    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_rec(&self.rect, Color::RED);
    }
}

struct FpsObject {
    fps: u32,
    x: i32,
    y: i32,
    fontsize: i32,
    color: Color,
}
impl FpsObject {
    fn new(x: i32, y: i32, fontsize: i32, color: Color) -> FpsObject {
        FpsObject {
            fps: 0,
            x: x,
            y: y,
            fontsize: fontsize,
            color: color
        }
    }
}
impl GameObject for FpsObject {
    fn destroy(&self) {}
    fn update(&mut self, rl: &RaylibHandle) {
        self.fps = rl.get_fps();
    }
    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_text(&self.fps.to_string(), self.x, self.y, self.fontsize, self.color);
    }
}

pub struct Game {
    pub config: Config,
    pub scene: Scene, //current scene
}

impl Game {
    pub fn new(config_path: &str) -> Game {
        let mut game = Game {
            config: Config::new(config_path),
            scene: Scene::new(),
        };
        let fps = FpsObject::new(12, 12, 20, Color::BLACK);
        game.scene.add_object(fps);

        game
    }

    //update all GameObjects in the scene
    pub fn update(&mut self, rl: &RaylibHandle) {
        for obj in &mut self.scene.objs {
            obj.update(&rl);
        }
    }

    //draw all GameObjects in the scene
    pub fn draw(&self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::WHITE);
        for obj in &self.scene.objs {
            obj.draw(&mut d);
        }
    }
}

//a scene is a collection of GameObjects that are bundled together
pub struct Scene {
    objs: Vec<Box<dyn GameObject>>, //a vec of structs implementing GameObject
}

impl Scene {
    fn new() -> Scene {
        Scene {
            objs: Vec::<Box<dyn GameObject>>::new(),
        }
    }
    fn add_object<T: 'static + GameObject>(&mut self, obj: T) {
        self.objs.push(Box::new(obj));
    }
}

//Struct for config data like game settings
#[derive(Debug, Deserialize)]
pub struct Config {
    pub size: [i32; 2],
    pub fullscreen: bool,
    pub vsync: bool,
    pub fps_target: u32,
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
        config
    }
}
