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
use std::time::Duration;
use std::time::Instant;

fn restart_game(ball: &mut Ball, l_player: &mut Player, r_player: &mut Player) {
    ball.set_pos(DEFAULT_BALL_POS);
    ball.set_x_vel(MIN_BALL_VEL);
    ball.set_y_vel(MIN_BALL_VEL);
    l_player.get_mut_paddle().set_y(PADDLE_CENTER);
    r_player.get_mut_paddle().set_y(PADDLE_CENTER);
}

fn draw_ball_move_count_down(count: i32) {
    draw_text_ex(
        count.to_string().as_str(),
        CENTER_X - (TEXT_SIZE as f32 / 2.),
        CENTER_Y,
        TextParams {
            font_size: TEXT_SIZE,
            color: PADDLE_COLOR,
            ..Default::default()
        },
    );
}

#[macroquad::main(set_conf)]
async fn main() {
    let l_paddle = Paddle::new(
        PADDLE_DIM,
        PADDLE_POS_L,
        PADDLE_VEL,
        PADDLE_POS_L.x + PADDLE_W,
        PADDLE_COLOR,
    );
    let r_paddle = Paddle::new(
        PADDLE_DIM,
        PADDLE_POS_R,
        PADDLE_VEL,
        PADDLE_POS_R.x,
        PADDLE_COLOR,
    );
    let l_score = Score::new(TEXT_SIZE, SCORE_POS_L, PADDLE_COLOR);
    let r_score = Score::new(TEXT_SIZE, SCORE_POS_R, PADDLE_COLOR);
    let mut l_player = Player::new(l_paddle, l_score, (KeyCode::W, KeyCode::S));
    let mut r_player = Player::new(r_paddle, r_score, (KeyCode::Up, KeyCode::Down));
    let mut ball = Ball::new(DEFAULT_BALL_POS, MIN_BALL_VEL, BALL_RADIUS, BALL_COLOR);

    // GAME RELATED BOOLS (Soon to be states via enum)
    let play_game: bool = true;
    let mut ball_spawn: bool = true;
    let mut count_down_time = Instant::now();
    let mut ball_move_count_down: i32 = BALL_COUNT_DOWN_TIME;

    loop {
        let delta_time = get_frame_time();

        clear_background(GRAY);
        if play_game {
            if ball_spawn {
                draw_ball_move_count_down(ball_move_count_down);
                if Instant::now() - count_down_time >= Duration::from_secs(1) {
                    ball_move_count_down -= 1;
                    count_down_time = Instant::now();
                    if ball_move_count_down < 0 {
                        ball.set_dir();
                        ball_move_count_down = BALL_COUNT_DOWN_TIME;
                        ball_spawn = false;
                    }
                }
            } else {
                ball.update(delta_time);
                l_player.update(delta_time);
                r_player.update(delta_time);

                bounce_ball_at_wall(&mut ball, screen_height());
                bounce_ball_on_paddle(&mut ball, l_player.get_paddle());
                bounce_ball_on_paddle(&mut ball, r_player.get_paddle());

                if ball.get_pos().x < l_player.get_paddle().get_x() {
                    r_player.score();
                    restart_game(&mut ball, &mut l_player, &mut r_player);
                    ball_spawn = true;
                    count_down_time = Instant::now();
                }

                if ball.get_pos().x > r_player.get_paddle().get_x() + PADDLE_W {
                    l_player.score();
                    restart_game(&mut ball, &mut l_player, &mut r_player);
                    ball_spawn = true;
                    count_down_time = Instant::now();
                }

                l_player.draw();
                r_player.draw();
                ball.draw();
            }
        }

        next_frame().await
    }
}
