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
    label: Label,
    shape: Rect,
    color: Color,
}

impl Button {
    pub fn new(_x: f32, _y: f32, _label: &str, sizew: f32, sizeh: f32, _color: u32) -> Button {
        let offx = sizew/2.0;
        let offy = sizeh/2.0;
        let labelchild = Label::new(_x+offx, _y+offy, _label, 0xffffff, Talign::Center);
        Button {
            pos: [_x, _y],
            label: labelchild,
            shape: Rect::new(_x, _y, sizew, sizeh),
            color: Color::from_rgba_u32(_color),
        }
    }

}

impl UI for Button {
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let rectangle =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.shape, self.color)?;
        //draw label
        self.label.draw(ctx);
        //draw button
        graphics::draw(ctx, &rectangle, (Point2::new(0.0, 0.0),))
    }
}

pub enum Talign {
    Left,
    Center
}

pub struct Label {
    label: Text,
    color: Color,
    pos: [f32; 2],
    alignment: Talign,
}

impl Label {
    pub fn new(_x: f32, _y: f32, _label: &str, _color: u32, align: Talign) -> Label {
        Label {
            pos: [_x, _y],
            color: Color::from_rgba_u32(_color),
            label: Text::new(_label.to_string()),
            alignment: align,
        }
    }
    pub fn set_text(&mut self, _label: &str) {
        self.label = Text::new(_label.to_string());
    }
}

impl UI for Label {
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let width = self.label.width(ctx) as f32;
        let height = self.label.height(ctx) as f32;
        let aligned = match self.alignment {
            Talign::Left => Point2::new(self.pos[0], self.pos[1]),
            Talign::Center => Point2::new(self.pos[0]-(width/2.0), self.pos[1]-(height/2.0)),
        };
        Ok(graphics::queue_text(ctx, &self.label, aligned, None))
    }
}

pub trait UI {
    fn draw(&self, ctx: &mut Context) -> GameResult<()>;
}
