use macroquad::prelude::*;
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
        draw_rectangle(
            self.hitbox.x,
            self.hitbox.y,
            self.hitbox.w,
            self.hitbox.h,
            BLACK,
        );
    }

    pub fn update(&mut self) {
        self.vel.x += gravity.x;
        self.vel.y += gravity.y;
        self.hitbox.x += self.vel.x * get_frame_time();
        self.hitbox.y += self.vel.y * get_frame_time()
    }
}
