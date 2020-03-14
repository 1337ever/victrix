use ggez::event::{self, EventHandler};
use ggez::graphics::*;
use ggez::nalgebra::Point2;
use ggez::*;
use std::collections::BTreeMap;

const PADDING: f32 = 10.0;

pub struct Window {
    uielements: BTreeMap<&'static str, Box<dyn UI>>,
    pos: [f32; 2],
    width: f32,
    height: f32,
}

impl Window {
    pub fn new(_x: f32, _y: f32) -> Window {
        Window {
            uielements: BTreeMap::new(),
            pos: [_x, _y],
            width: 200.0,
            height: 10.0,
        }
    }
    //calculate window size and the position of elements in the window
    pub fn layout_window(&mut self) {
        let mut i = 0;
        for (_key, elm) in &self.uielements {
            //get the initial position of the window
            let mut px = self.pos[0];
            let mut py = self.pos[1];
            //get the size of the element
            let (sx, sy) = match elm.get_size() {
                Some(v) => v,
                None => (0.0, 20.0),
            };
            px+=PADDING;
            py+=( (sy/2.0)*i as f32 )+PADDING;
            
            i+=1;
        }
    }
    
    //insert an element into the window
    pub fn insert_elm(&mut self, index: &'static str, elm: Box<dyn UI>) {
        self.uielements.insert(index, elm);
        self.layout_window();
    }
}

pub struct Button {
    pos: [f32; 2],
    label: Label,
    shape: Rect,
    color: Color,
    clicked: bool,
}

impl Button {
    pub fn new(_x: f32, _y: f32, _label: &str, sizew: f32, sizeh: f32, _color: u32) -> Button {
        let offx = sizew / 2.0;
        let offy = sizeh / 2.0;
        //create a label centered on the button
        let labelchild = Label::new(_x + offx, _y + offy, _label, 0xffffff, Talign::Center);
        Button {
            pos: [_x, _y],
            label: labelchild,
            shape: Rect::new(_x, _y, sizew, sizeh),
            color: Color::from_rgba_u32(_color),
            clicked: false,
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
    fn get_size(&self) -> Option<(f32, f32)> {
        Some((self.shape.w, self.shape.h))
    }
}

//enum for text alignment
pub enum Talign {
    Left,
    Center,
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
        //align text
        let width = self.label.width(ctx) as f32;
        let height = self.label.height(ctx) as f32;
        let aligned = match self.alignment {
            Talign::Left => Point2::new(self.pos[0], self.pos[1]),
            Talign::Center => {
                Point2::new(self.pos[0] - (width / 2.0), self.pos[1] - (height / 2.0))
            }
        };

        //queue text
        Ok(graphics::queue_text(ctx, &self.label, aligned, None))
    }
    fn get_size(&self) -> Option<(f32, f32)> {
        None
    }
}

//trait for UI elements
pub trait UI {
    fn draw(&self, ctx: &mut Context) -> GameResult<()>;
    fn get_size(&self) -> Option<(f32, f32)>;
}
