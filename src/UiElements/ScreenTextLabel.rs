use macroquad::{color, prelude::*};

pub struct Textlabel {
    pub properties: Rect,
    pub pos: Vec2,
    pub size: Vec2,

    pub Text: String,
    pub FontSize: f32,
    pub TextColor: Color,
    pub Text_Transparency: f32,
}

impl Textlabel {
    pub fn new(
        pos: Vec2,
        size: Vec2,
        Text: String,
        FontSize: f32,
        TextColor: Color,
        Text_Transparency: f32,
    ) -> Self {
        Self {
            properties: Rect::new(pos.x, pos.y, size.x, size.y),
            pos,
            size,

            Text,
            FontSize,
            TextColor,
            Text_Transparency,
        }
    }

    pub fn draw(&self) {
        draw_text(
            self.Text.as_str(),
            self.pos.x,
            self.pos.y,
            self.FontSize,
            Color {
                r: (self.TextColor.r),
                g: (self.TextColor.g),
                b: (self.TextColor.b),
                a: (self.Text_Transparency),
            },
        );
    }
}
