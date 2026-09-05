mod UiElements;
mod player;

use UiElements::ScreenWindow;
use macroquad::prelude::*;
use player::Player;
const WORLD_SIZE: Vec2 = vec2(1080., 500.);

#[macroquad::main("game")]
async fn main() {
    set_pc_assets_folder("assets");

    let image = load_image("character.png").await.unwrap();
    let texture = Texture2D::from_image(&image);
    let mut player = Player::new(vec2(0., 0.), vec2(0., 0.));

    let border = Rect::new(
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
        if is_key_down(KeyCode::A) {
            player.vel.x = -200.
        } else if is_key_down(KeyCode::D) {
            player.vel.x = 200.
        } else {
            player.vel.x = 0.
        }

        if is_key_down(KeyCode::W) && player.on_ground {
            player.on_ground = false;
            player.vel.y = -200.
        }

        player.update();

        if player.hitbox.bottom() >= border.bottom() {
            player.hitbox.y = border.bottom() - player.hitbox.h;
            player.on_ground = true
        } else if player.hitbox.top() <= border.top() {
            player.hitbox.y = border.top();
        }

        if player.hitbox.left() <= border.left() {
            player.hitbox.x = border.left();
        } else if player.hitbox.right() >= border.right() {
            player.hitbox.x = border.right() - player.hitbox.w;
        }

        draw_rectangle_lines(border.x, border.y, border.w, border.h, 5., BLUE);
        player.texturedraw(&texture);
        player.draw();

        next_frame().await
    }
}
