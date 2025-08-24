mod ball;
mod collisions;
mod game_traits;
mod paddle;
mod score;
mod settings;

use crate::ball::*;
use crate::collisions::*;
use crate::game_traits::*;
use crate::paddle::*;
use crate::settings::*;

use macroquad::prelude::*;

#[macroquad::main(set_conf)]
async fn main() {
    let mid_screen_y: f32 = screen_height() / 2.0;
    let mid_screen_x: f32 = screen_width() / 2.0;
    let game_start_player_y_coord = mid_screen_y - (PADDLE_HEIGHT / 2.0);
    let play_game: bool = true;

    let mut left_player = init_paddle(
        Vec2 {
            x: PADDLE_X_OFFSET,
            y: game_start_player_y_coord,
        },
        PADDLE_VELOCITY,
        (KeyCode::S, KeyCode::W),
        PADDLE_X_OFFSET + PADDLE_WIDTH,
    );

    let mut right_player = init_paddle(
        Vec2 {
            x: screen_width() - PADDLE_WIDTH - PADDLE_X_OFFSET,
            y: game_start_player_y_coord,
        },
        PADDLE_VELOCITY,
        (KeyCode::Down, KeyCode::Up),
        screen_width() - PADDLE_WIDTH - PADDLE_X_OFFSET,
    );

    let mut ball = init_ball(
        Vec2 {
            x: mid_screen_x,
            y: mid_screen_y,
        },
        Vec2 {
            x: MINIMUM_BALL_VELOCITY,
            y: MINIMUM_BALL_VELOCITY,
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
