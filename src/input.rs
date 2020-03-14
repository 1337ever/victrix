use ggez;
use ggez::input;
use ggez::event::{self, Axis, Button, GamepadId, KeyCode, KeyMods, MouseButton};

pub struct Mouse {
    pub x: f32,
    pub y: f32,
    pub mouse_down: bool,
}

impl Mouse {
    fn new() -> Mouse {
        Mouse {
            x: 0.0,
            y: 0.0,
            mouse_down: false,
        }
    }
}

pub struct Keyboard {

}

impl Keyboard {
    fn new() -> Keyboard {
        Keyboard {}
    }
}

pub struct Input {
    pub mouse: Mouse,
    pub keyboard: Keyboard,
}

impl Input {
    pub fn new() -> Input {
        Input {
            mouse: Mouse::new(),
            keyboard: Keyboard::new(),
        }
    }
}
