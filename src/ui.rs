use ggez::*;
use ggez::event::{self, EventHandler};
use ggez::graphics::*;

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

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let rectangle =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.shape, self.color)?;
        graphics::draw(ctx, &rectangle, (ggez::nalgebra::Point2::new(0.0, 0.0),))
    }
}

pub trait Draw {
    fn draw(&self, ctx: &mut Context);
}
