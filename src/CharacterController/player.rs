

use std::collections::HashMap;

use macroquad::{miniquad::TextureParams, prelude::*};

use crate::ScreenWindow::Window;

use crate::CharacterController::Animation::*;
const HitboxSize: Vec2 = vec2(40f32, 100f32);
const gravity: Vec2 = vec2(0., 8.);
pub struct Player {
    pub hitbox: Rect,
    pub pos: Vec2,
    pub on_ground: bool,
    pub vel: Vec2,
    pub animctrl: AnimationController,
    pub flip : bool
}

impl Player {
    pub fn new(pos: Vec2, vel: Vec2) -> Self {
        Self {
            hitbox: Rect::new(pos.x, pos.y, HitboxSize.x, HitboxSize.y),
            pos,
            vel,
            on_ground: false,
            animctrl: AnimationController::new(),
            flip: false
        }
    }

    pub fn draw(&mut self,textures: &HashMap<String, Texture2D>) {

        
        self.animctrl.update(get_frame_time(), textures,  self.hitbox, self.flip);
        
        draw_rectangle_lines(
            self.hitbox.x,
            self.hitbox.y,
            self.hitbox.w,
            self.hitbox.h,
            5.,
            BLACK,
        );

    }



    pub fn update(&mut self, world: &Rect) {

        //input
        if is_key_down(KeyCode::A) {
            self.animctrl.set_animation("walk", 10, 0.1,5);
            self.vel.x = -200.;
            self.flip = true
        } else if is_key_down(KeyCode::D) {
            self.animctrl.set_animation("walk", 10, 0.1,5);
            self.vel.x = 200.;
            self.flip = false
        } else {
            self.animctrl.set_animation("idle", 5, 0.1,2);
            self.vel.x = 0.
        }


        //update
        self.vel.x += gravity.x;
        self.vel.y += gravity.y;
        self.hitbox.x += self.vel.x * get_frame_time();
        self.hitbox.y += self.vel.y * get_frame_time();

        //collision
         if self.hitbox.bottom() >= world.bottom() {
            self.hitbox.y = world.bottom() - self.hitbox.h;
            self.on_ground = true
        } else if self.hitbox.top() <= world.top() {
            self.hitbox.y = world.top();
        }

        if self.hitbox.left() <= world.left() {
            self.hitbox.x = world.left();
        } else if self.hitbox.right() >= world.right() {
            self.hitbox.x = world.right() - self.hitbox.w;
        }

        if is_key_down(KeyCode::W) && self.on_ground {
            self.on_ground = false;
            self.vel.y = -200.
        }

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
