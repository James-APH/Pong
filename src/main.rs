mod ball;
mod collisions;
mod gamestate;
mod paddle;
mod pong_types;
mod score;
mod settings;

use crate::ball::init_ball;
use crate::collisions::*;
use crate::paddle::*;
use crate::pong_types::*;

use macroquad::prelude::*;

#[macroquad::main("Pong")]
async fn main() {
    let player_speed: f32 = 250.;
    let player_x_offset: f32 = 16.;
    let mid_screen_y: f32 = screen_height() / 2.0;
    let mid_screen_x: f32 = screen_width() / 2.0;
    let play_game: bool = true;
    // LEFT PLAYER
    let mut p1 = init_paddle(
        Vec2 {
            x: player_x_offset,
            y: mid_screen_y - (PADDLE_HEIGHT / 2.0),
        },
        player_speed,
        (KeyCode::S, KeyCode::W),
    );
    // RIGHT PLAYER
    let mut p2 = init_paddle(
        Vec2 {
            x: screen_width() - PADDLE_WIDTH - player_x_offset,
            y: mid_screen_y - (PADDLE_HEIGHT / 2.0),
        },
        player_speed,
        (KeyCode::Down, KeyCode::Up),
    );
    // BALL
    let mut ball = init_ball(
        Vec2 {
            x: mid_screen_x,
            y: mid_screen_y,
        },
        Vec2 { x: 20.0, y: 20.0 },
        Vec2 { x: 1.0, y: -1.0 },
    );

    // main game loop
    loop {
        let delta_time = get_frame_time();

        clear_background(GRAY);
        if play_game {
            ball.update(delta_time);
            p1.update(delta_time);
            p2.update(delta_time);

            bounce_ball_at_wall(&mut ball, screen_height());
            is_ball_at_paddle(&ball, &p1);
            bounce_ball(&mut ball, &p1);
            is_ball_at_paddle(&ball, &p2);
            bounce_ball(&mut ball, &p2);
        }

        p1.draw();
        p2.draw();
        ball.draw();
        next_frame().await
    }
}
