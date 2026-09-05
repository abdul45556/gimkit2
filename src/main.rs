mod ScreenWindow;
mod UiElements;
mod player;

use macroquad::prelude::*;

use ScreenWindow::Window;
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

    loop {
        let camera = Camera2D {
            target: vec2(player.hitbox.x, player.hitbox.y + 5.),
            zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
            ..Default::default()
        };
        set_camera(&camera);
        clear_background(WHITE);

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

        let mut window = player.CreateWindow(vec2(0., 0.), vec2(1000., 500.), BLACK, 5.);
        let mut frame = window.CreateFrame(vec2(0., 0.), vec2(1000., 500.), BLUE, DARKBLUE, 1., 5.);
        let mut button = frame.CreateButton(
            vec2(0., 0.),
            vec2(500., 200.),
            DARKBLUE,
            BLACK,
            1.,
            5.,
            &camera,
        );
        let textlabel = frame.CreateTextlabel(
            vec2(-300., 0.),
            vec2(0., 200.),
            String::from("QUESTIONS"),
            100.,
            BLACK,
            1.,
        );
        if button.button_pressed(MouseButton::Left) {
            println!("pressed")
        }

        draw_rectangle_lines(border.x, border.y, border.w, border.h, 1., BLUE);
        player.texturedraw(&texture);
        player.draw();
        window.draw();
        frame.draw();
        button.draw();
        textlabel.draw();

        next_frame().await
    }
}
