mod ball;
mod collisions;
mod paddle;
mod pong_types;
mod score;

use crate::ball::*;
use crate::collisions::*;
use crate::paddle::*;
use crate::pong_types::*;

use macroquad::prelude::*;

fn set_conf() -> Conf {
    Conf {
        window_title: String::from("PONG ULTIMATE BALL OUT"),
        window_width: 1000,
        window_height: 500,
        ..Default::default()
    }
}

#[macroquad::main(set_conf)]
async fn main() {
    const PLAYER_VEL: f32 = 250.;
    const INITIAL_BALL_VEL: f32 = 75.;
    const PLAYER_X_OFFSET: f32 = 16.;
    let mid_screen_y: f32 = screen_height() / 2.0;
    let mid_screen_x: f32 = screen_width() / 2.0;
    let game_start_player_y_coord = mid_screen_y - (PADDLE_HEIGHT / 2.0);
    let play_game: bool = true;

    let mut left_player = init_paddle(
        Vec2 {
            x: PLAYER_X_OFFSET,
            y: game_start_player_y_coord,
        },
        PLAYER_VEL,
        (KeyCode::S, KeyCode::W),
        PLAYER_X_OFFSET + PADDLE_WIDTH,
    );

    let mut right_player = init_paddle(
        Vec2 {
            x: screen_width() - PADDLE_WIDTH - PLAYER_X_OFFSET,
            y: game_start_player_y_coord,
        },
        PLAYER_VEL,
        (KeyCode::Down, KeyCode::Up),
        screen_width() - PADDLE_WIDTH - PLAYER_X_OFFSET,
    );

    let mut ball = init_ball(
        Vec2 {
            x: mid_screen_x,
            y: mid_screen_y,
        },
        Vec2 {
            x: INITIAL_BALL_VEL,
            y: INITIAL_BALL_VEL,
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
