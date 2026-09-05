use macroquad::{
    color,
    prelude::{camera::mouse, *},
};

use crate::UiElements::ScreenTextLabel::Textlabel;

pub struct Button {
    pub properties: Rect,
    pub pos: Vec2,
    pub mousepos: Vec2,
    pub size: Vec2,
    pub BackgroundColor: Color,
    pub StrokeColor: Color,
    pub StrokeThickness: f32,
    pub Transparency: f32,
}

impl Button {
    pub fn new(
        pos: Vec2,

        size: Vec2,
        BackgroundColor: Color,
        StrokeColor: Color,
        StrokeThickness: f32,
        Transparency: f32,
        camera: &Camera2D,
    ) -> Self {
        let (mx, my) = mouse_position();
        let mouse_pos = vec2(mx, my);
        let newpos = camera.screen_to_world(mouse_pos);
        Self {
            properties: Rect::new(pos.x, pos.y, size.x, size.y),
            pos,
            mousepos: newpos,
            size,
            BackgroundColor,
            StrokeColor,
            Transparency,
            StrokeThickness,
        }
    }

    pub fn draw(&mut self) {
        if self.properties.contains(self.mousepos) {
            self.BackgroundColor.r = (self.BackgroundColor.r - 0.1).max(0.0);
            self.BackgroundColor.g = (self.BackgroundColor.g - 0.1).max(0.0);
            self.BackgroundColor.b = (self.BackgroundColor.b - 0.1).max(0.0);
        }
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

    pub fn button_pressed(&mut self, MouseButton: MouseButton) -> bool {
        if self.properties.contains(self.mousepos) && is_mouse_button_pressed(MouseButton) {
            true
        } else {
            false
        }
    }
}
