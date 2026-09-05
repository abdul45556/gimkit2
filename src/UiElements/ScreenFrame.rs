use macroquad::{color, prelude::*};

use crate::UiElements::ScreenButton::Button;
use crate::UiElements::ScreenTextLabel::Textlabel;

pub struct Frame {
    pub properties: Rect,
    pub pos: Vec2,
    pub size: Vec2,
    pub BackgroundColor: Color,
    pub StrokeColor: Color,
    pub StrokeThickness: f32,
    pub Transparency: f32,
}

impl Frame {
    pub fn new(
        pos: Vec2,
        size: Vec2,
        BackgroundColor: Color,
        StrokeColor: Color,
        StrokeThickness: f32,
        Transparency: f32,
    ) -> Self {
        Self {
            properties: Rect::new(pos.x, pos.y, size.x, size.y),
            pos,
            size,
            BackgroundColor,
            StrokeColor,
            Transparency,
            StrokeThickness,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.pos.x,
            self.pos.y,
            self.size.x,
            self.size.y,
            Color {
                r: (self.BackgroundColor.r),
                g: (self.BackgroundColor.g),
                b: (self.BackgroundColor.b),
                a: (self.Transparency),
            },
        );
        draw_rectangle_lines(
            self.pos.x,
            self.pos.y,
            self.size.x,
            self.size.y,
            self.StrokeThickness,
            Color {
                r: (self.StrokeColor.r),
                g: (self.StrokeColor.g),
                b: (self.StrokeColor.b),
                a: (self.Transparency),
            },
        );
    }

    pub fn CreateTextlabel(
        &mut self,
        pos: Vec2,
        size: Vec2,

        Text: String,
        FontSize: f32,
        TextColor: Color,
        Text_Transparency: f32,
    ) -> Textlabel {
        Textlabel::new(
            vec2(
                self.properties.center().x - size.x / 2. + pos.x,
                self.properties.center().y - size.y / 2. + pos.y,
            ),
            size,
            Text,
            FontSize,
            TextColor,
            Text_Transparency,
        )
    }

    pub fn CreateButton(
        &mut self,
        pos: Vec2,
        size: Vec2,
        BackgroundColor: Color,
        StrokeColor: Color,
        Transparency: f32,
        StrokeThickness: f32,
        camera: &Camera2D,
    ) -> Button {
        Button::new(
            vec2(
                self.properties.center().x - size.x / 2. + pos.x,
                self.properties.center().y - size.y / 2. + pos.y,
            ),
            size,
            BackgroundColor,
            StrokeColor,
            StrokeThickness,
            Transparency,
            camera,
        )
    }
}
