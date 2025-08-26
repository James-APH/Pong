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
use crate::score::Score;
use crate::settings::*;
use macroquad::prelude::*;

#[macroquad::main(set_conf)]
async fn main() {
    let score_dim = 50;
    let screen_h = SCREEN_H as f32;
    let screen_w = SCREEN_W as f32;
    let center_y = screen_h / 2.0;
    let center_x = screen_w / 2.0;
    let paddle_center = center_y - PADDLE_H / 2.0;
    let paddle_dim = Vec2::new(PADDLE_W, PADDLE_H);
    let paddle_pos_l = Vec2::new(PADDLE_X_OFFSET, paddle_center);
    let paddle_pos_r = Vec2::new(screen_w - PADDLE_W - PADDLE_X_OFFSET, paddle_center);
    let score_pos_l = Vec2::new(score_dim as f32 * 2., score_dim as f32 * 2.);
    let score_pos_r = Vec2::new(screen_w - score_dim as f32 * 2., score_dim as f32 * 2.);
    let ball_pos = Vec2::new(center_x, center_y);
    let ball_dir = Vec2::new(1.0, 1.0);

    let l_paddle = Paddle::new(
        paddle_dim,
        paddle_pos_l,
        PADDLE_VEL,
        paddle_pos_l.x + PADDLE_W,
        PADDLE_COLOR,
    );
    let r_paddle = Paddle::new(
        paddle_dim,
        paddle_pos_r,
        PADDLE_VEL,
        paddle_pos_r.x,
        PADDLE_COLOR,
    );
    let l_score = Score::new(score_dim, score_pos_l, PADDLE_COLOR);
    let r_score = Score::new(score_dim, score_pos_r, PADDLE_COLOR);

    let mut ball = Ball::new(ball_pos, MIN_BALL_VEL, ball_dir, BALL_RADIUS, BALL_COLOR);

    let mut l_player = Player::new("LEFT", l_paddle, l_score, (KeyCode::W, KeyCode::S));
    let mut r_player = Player::new("RIGHT", r_paddle, r_score, (KeyCode::Up, KeyCode::Down));

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
