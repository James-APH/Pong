mod ball;
mod gamestate;
mod paddle;
mod pong_types;
mod score;
use macroquad::prelude::*;

use crate::{paddle::init_paddle, pong_types::Draw, pong_types::Update};

#[macroquad::main("Pong")]
async fn main() {
    let player_width: f32 = 32.;
    let player_height: f32 = 128.;
    let player_dimensions: (f32, f32) = (player_width, player_height);
    let player_speed: f32 = 250.;
    let player_x_offset: f32 = 16.;
    let player_y_coord: f32 = screen_height() / 2.0;
    let play_game: bool = true;
    // will be left player
    let mut p1 = init_paddle(
        player_dimensions,
        (
            player_x_offset,
            (screen_height() / 2.0) - (player_height / 2.0),
        ),
        player_speed,
        (KeyCode::S, KeyCode::W),
        BLUE,
    );
    // will be right player
    let mut p2 = init_paddle(
        player_dimensions,
        (
            screen_width() - player_width - player_x_offset,
            (screen_height() / 2.0) - (player_height / 2.0),
        ),
        player_speed,
        (KeyCode::Down, KeyCode::Up),
        BLUE,
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
