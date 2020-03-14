use ggez::*;
use ggez::event::{self, EventHandler};
use ggez::graphics::*;
use ggez::nalgebra::Point2;
use std::collections::BTreeMap;

pub struct Window {
    uielements: BTreeMap<&'static str, Box::<dyn UI>>,
    pos: [f32; 2],
}

pub struct Button {
    pos: [f32; 2],
    label: Text,
    shape: Rect,
    color: Color,
}

impl Button {
    pub fn new(_x: f32, _y: f32, _label: &str, sizew: f32, sizeh: f32, _color: u32) -> Button {
        Button {
            pos: [_x, _y],
            label: Text::new(_label.to_string()),
            shape: Rect::new(_x/2.0, _y/2.0, sizew, sizeh),
            color: Color::from_rgba_u32(_color),
        }
    }

}

impl UI for Button {
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let rectangle =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.shape, self.color)?;
        graphics::queue_text(ctx, &self.label, Point2::new(self.pos[0], self.pos[1]), None);
        graphics::draw(ctx, &rectangle, (Point2::new(0.0, 0.0),))
    }
}

pub struct Label {
    label: Text,
    color: Color,
    pos: [f32; 2],
}

impl Label {
    pub fn new(_x: f32, _y: f32, _label: &str, _color: u32) -> Label {
        Label {
            pos: [_x, _y],
            color: Color::from_rgba_u32(_color),
            label: Text::new(_label.to_string()),
        }
    }
    pub fn set_text(&mut self, _label: &str) {
        self.label = Text::new(_label.to_string());
    }
}

impl UI for Label {
   fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        Ok(graphics::queue_text(ctx, &self.label, Point2::new(self.pos[0], self.pos[1]), None))
    }
}

pub trait UI {
    fn draw(&self, ctx: &mut Context) -> GameResult<()>;
}
