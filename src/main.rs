mod ball;
mod gamestate;
mod paddle;
mod pong_types;
mod score;
mod settings;

use crate::paddle::*;
use crate::pong_types::*;
use macroquad::prelude::*;

#[macroquad::main("Pong")]
async fn main() {
    let player_speed: f32 = 250.;
    let player_x_offset: f32 = 16.;
    let player_y_coord: f32 = screen_height() / 2.0;
    let play_game: bool = true;
    // LEFT PLAYER
    let mut p1 = init_paddle(
        Vec2 {
            x: player_x_offset,
            y: player_y_coord - (PADDLE_HEIGHT / 2.0),
        },
        player_speed,
        (KeyCode::S, KeyCode::W),
    );
    // RIGHT PLAYER
    let mut p2 = init_paddle(
        Vec2 {
            x: screen_width() - PADDLE_WIDTH - player_x_offset,
            y: player_y_coord - (PADDLE_HEIGHT / 2.0),
        },
        player_speed,
        (KeyCode::Down, KeyCode::Up),
    );

    // main game loop
    loop {
        let delta_time = get_frame_time();

        clear_background(GRAY);
        if play_game {
            p1.update(delta_time);
            p2.update(delta_time);
        }

        p1.draw();
        p2.draw();

        next_frame().await
    }
}
