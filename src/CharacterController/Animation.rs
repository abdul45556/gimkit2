use std::collections::HashMap;

use macroquad::prelude::*;

pub struct AnimationController {
    pub current: String,
    pub current_frame: usize,
    pub timer: f32,
    pub frame_time: f32,
    pub frame_count: usize,
    pub columns: usize,
}

impl AnimationController {
    pub fn new() -> Self {
        Self {
            current: "walk".to_string(),
            current_frame: 0,
            timer: 0.0,
            frame_time: 0.1,
            frame_count: 10,
            columns: 5,
        }
    }

    pub fn update(&mut self, dt: f32, textures: &HashMap<String, Texture2D>, hitbox: Rect, flip: bool) {
        self.timer += dt;
        if self.timer >= self.frame_time {
            self.timer -= self.frame_time;
            self.current_frame += 1;
            if self.current_frame >= self.frame_count {
                self.current_frame = 0;
            }
        }
    println!("{}",self.current_frame);
        let Some(texture) = textures.get(&self.current) else { return };

        let draw_size = vec2(150., 150.);
        let col = self.current_frame % self.columns;
        let row = self.current_frame / self.columns;

        draw_texture_ex(
            texture,
            hitbox.x - (draw_size.x - hitbox.w) / 2.0,
            hitbox.y - (draw_size.y - hitbox.h) / 2.0,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    col as f32 * 512.0,
                    row as f32 * 512.0,
                    512.0,
                    512.0,
                )),
                dest_size: Some(draw_size),
                flip_x: flip,
                ..Default::default()
            },
        );
    }

    pub fn set_animation(&mut self, animation: &str, frame_count: usize, frame_time: f32, columns: usize) {
        if self.current == animation {
            return;
        }
        self.current = animation.to_string();
        self.current_frame = 0;
        self.timer = 0.0;
        self.frame_count = frame_count;
        self.frame_time = frame_time;
        self.columns = columns;
    }
}   