use macroquad::{miniquad::TextureParams, prelude::*};

use crate::ScreenWindow::Window;
const HitboxSize: Vec2 = vec2(40f32, 100f32);
const gravity: Vec2 = vec2(0., 8.);
pub struct Player {
    pub hitbox: Rect,
    pub pos: Vec2,
    pub on_ground: bool,
    pub vel: Vec2,
}

impl Player {
    pub fn new(pos: Vec2, vel: Vec2) -> Self {
        Self {
            hitbox: Rect::new(pos.x, pos.y, HitboxSize.x, HitboxSize.y),
            pos,
            vel,
            on_ground: false,
        }
    }

    pub fn draw(&self) {
        draw_rectangle_lines(
            self.hitbox.x,
            self.hitbox.y,
            self.hitbox.w,
            self.hitbox.h,
            5.,
            BLACK,
        );
    }

    pub fn texturedraw(&self, texture: &Texture2D) {
        draw_texture(&texture, self.hitbox.x, self.hitbox.y, WHITE);
    }

    pub fn update(&mut self) {
        self.vel.x += gravity.x;
        self.vel.y += gravity.y;
        self.hitbox.x += self.vel.x * get_frame_time();
        self.hitbox.y += self.vel.y * get_frame_time()
    }

    pub fn CreateWindow(
        &mut self,
        pos: Vec2,
        size: Vec2,

        StrokeColor: Color,
        StrokeThickness: f32,
    ) -> Window {
        Window::new(
            vec2(
                self.hitbox.center().x - size.x / 2. + pos.x,
                self.hitbox.center().y - size.y / 2. + pos.y,
            ),
            size,
            StrokeColor,
            StrokeThickness,
        )
    }
}
