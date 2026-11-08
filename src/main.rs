mod UiElements;
mod CharacterController;

use std::collections::HashMap;

use UiElements::ScreenWindow;
use macroquad::prelude::*;
use CharacterController::*;

use crate::CharacterController::{player::Player};

const WORLD_SIZE: Vec2 = vec2(1080., 500.);

#[macroquad::main("game")]
async fn main() {
    set_pc_assets_folder("assets");

  

let mut textures: HashMap<String, Texture2D> = HashMap::new();
textures.insert(
    "walk".to_string(),
    Texture2D::from_image(&load_image("Characters/Blobby/Walk3.png").await.unwrap()),
);
textures.insert(
    "idle".to_string(),
    Texture2D::from_image(&load_image("Characters/Blobby/idle2.png").await.unwrap()),
);

    let mut player = Player::new(vec2(0., 0.), vec2(0., 0.));

    let world = Rect::new(
        -WORLD_SIZE.x / 2.,
        -WORLD_SIZE.y / 2.,
        WORLD_SIZE.x,
        WORLD_SIZE.y,
    );
    let mut camera = Camera2D {
        target: vec2(player.hitbox.x, player.hitbox.y + 5.),
        zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
        ..Default::default()
    };

    loop {
        camera.target = vec2(player.hitbox.x, player.hitbox.y + 5.);
        camera.zoom = vec2(2.0 / screen_width(), 2.0 / screen_height());
        set_camera(&camera);

        let fps = get_fps();

        clear_background(WHITE);
        draw_text(&format!("FPS: {}", fps), 0., 0., 50., BLACK);


        player.update(&world);



        draw_rectangle_lines(world.x, world.y, world.w, world.h, 5., BLUE);
     
        player.draw(&textures);

        next_frame().await
    }
}
