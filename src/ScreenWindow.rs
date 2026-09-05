use crate::UiElements::ScreenFrame::*;
use macroquad::prelude::*;

pub struct Window {
    pub properties: Rect,
    pub pos: Vec2,
    pub size: Vec2,

    pub StrokeColor: Color,
    pub StrokeThickness: f32,
}

impl Window {
    pub fn new(pos: Vec2, size: Vec2, StrokeColor: Color, StrokeThickness: f32) -> Self {
        Self {
            properties: Rect::new(pos.x, pos.y, size.x, size.y),
            pos,
            size,

            StrokeColor,
            StrokeThickness,
        }
    }

    pub fn draw(&self) {
        draw_rectangle_lines(
            self.pos.x,
            self.pos.y,
            self.size.x,
            self.size.y,
            self.StrokeThickness,
            self.StrokeColor,
        );
    }

    pub fn CreateFrame(
        &mut self,
        pos: Vec2,
        size: Vec2,
        BackgroundColor: Color,
        StrokeColor: Color,
        Transparency: f32,
        StrokeThickness: f32,
    ) -> Frame {
        Frame::new(
            vec2(
                self.properties.center().x - size.x / 2. + pos.x,
                self.properties.center().y - size.y / 2. + pos.y,
            ),
            size,
            BackgroundColor,
            StrokeColor,
            StrokeThickness,
            Transparency,
        )
    }
}
