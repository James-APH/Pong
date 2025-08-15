mod ball;
mod collisions;
mod gamestate;
mod paddle;
mod pong_types;
mod score;
mod settings;

use crate::ball::*;
use crate::collisions::*;
use crate::paddle::*;
use crate::pong_types::*;

use macroquad::prelude::*;

#[macroquad::main("Pong")]
async fn main() {
    let player_vel: f32 = 250.;
    let initial_ball_vel: f32 = 75.;
    let player_x_offset: f32 = 16.;
    let mid_screen_y: f32 = screen_height() / 2.0;
    let mid_screen_x: f32 = screen_width() / 2.0;
    let game_start_player_y_coord = mid_screen_y - (PADDLE_HEIGHT / 2.0);
    let play_game: bool = true;

    let mut left_player = init_paddle(
        Vec2 {
            x: player_x_offset,
            y: game_start_player_y_coord,
        },
        player_vel,
        (KeyCode::S, KeyCode::W),
    );

    let mut right_player = init_paddle(
        Vec2 {
            x: screen_width() - PADDLE_WIDTH - player_x_offset,
            y: game_start_player_y_coord,
        },
        player_vel,
        (KeyCode::Down, KeyCode::Up),
    );

    let mut ball = init_ball(
        Vec2 {
            x: mid_screen_x,
            y: mid_screen_y,
        },
        Vec2 {
            x: initial_ball_vel,
            y: initial_ball_vel,
        },
        Vec2 { x: 1.0, y: 1.0 },
    );

    // main game loop
    loop {
        let delta_time = get_frame_time();

        clear_background(GRAY);
        if play_game {
            ball.update(delta_time);
            left_player.update(delta_time);
            right_player.update(delta_time);

            bounce_ball_at_wall(&mut ball, screen_height());
            bounce_ball_on_paddle(&mut ball, &left_player);
            bounce_ball_on_paddle(&mut ball, &right_player);
        }

        left_player.draw();
        right_player.draw();
        ball.draw();
        next_frame().await
    }
}
