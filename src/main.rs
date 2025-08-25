mod ball;
mod collisions;
mod game_traits;
mod paddle;
mod player;
mod score;
mod settings;

use crate::ball::*;
use crate::collisions::*;
use crate::game_traits::*;
use crate::paddle::*;
use crate::player::*;
use crate::settings::*;
use macroquad::prelude::*;

#[macroquad::main(set_conf)]
async fn main() {
    let center_y = screen_height() / 2.0;
    let center_x = screen_width() / 2.0;
    let paddle_dim = Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT);
    let paddle_pos_l = Vec2::new(PADDLE_X_OFFSET, center_y - (PADDLE_HEIGHT / 2.0));
    let paddle_pos_r = Vec2::new(
        screen_width() - PADDLE_WIDTH - PADDLE_X_OFFSET,
        center_y - (PADDLE_HEIGHT / 2.0),
    );

    let l_paddle = Paddle::new(
        paddle_dim,
        paddle_pos_l,
        PADDLE_VELOCITY,
        paddle_pos_l.x + PADDLE_WIDTH,
    );
    let r_paddle = Paddle::new(
        paddle_dim,
        paddle_pos_r,
        PADDLE_VELOCITY,
        paddle_pos_r.x - PADDLE_WIDTH,
    );

    let mut l_player = Player::new("LEFT", l_paddle, (KeyCode::W, KeyCode::S));
    let mut r_player = Player::new("RIGHT", r_paddle, (KeyCode::Up, KeyCode::Down));

    let ball_pos = Vec2::new(center_x, center_y);
    let ball_dir = Vec2::new(1.0, 1.0);
    let mut ball = Ball::new(ball_pos, MINIMUM_BALL_VELOCITY, ball_dir);

    // main game loop
    let play_game: bool = true;

    loop {
        let delta_time = get_frame_time();

        clear_background(GRAY);
        if play_game {
            ball.update(delta_time);
            l_player.update(delta_time);
            r_player.update(delta_time);

            bounce_ball_at_wall(&mut ball, screen_height());
            bounce_ball_on_paddle(&mut ball, l_player.get_paddle());
            bounce_ball_on_paddle(&mut ball, r_player.get_paddle());
        }

        l_player.draw();
        r_player.draw();
        ball.draw();
        next_frame().await
    }
}
