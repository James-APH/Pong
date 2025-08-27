mod ball;
mod button;
mod collisions;
mod game_traits;
mod paddle;
mod player;
mod score;
mod settings;

use crate::ball::*;
use crate::button::SimpleButton;
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
    l_player.reset_score();
    r_player.reset_score();
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

enum GameState {
    Title,
    BallSpawn,
    GamePlay,
    Winner,
    Restart,
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
    let mut l_player = Player::new("LEFT", l_paddle, l_score, (KeyCode::W, KeyCode::S));
    let mut r_player = Player::new("RIGHT", r_paddle, r_score, (KeyCode::Up, KeyCode::Down));
    let mut ball = Ball::new(DEFAULT_BALL_POS, MIN_BALL_VEL, BALL_RADIUS, BALL_COLOR);
    let play_button = SimpleButton::new(
        "PLAY",
        BUTTON_DIM,
        Vec2 {
            x: 100.,
            y: BUTTON_Y,
        },
        GREEN,
    );
    let quit_button = SimpleButton::new(
        "QUIT",
        BUTTON_DIM,
        Vec2 {
            x: SCREEN_W - 100. - BUTTON_DIM.x,
            y: BUTTON_Y,
        },
        RED,
    );

    // GAME RELATED VARS
    let mut game_state = GameState::Title;

    let mut count_down_time = Instant::now();
    let mut ball_move_count_down: i32 = BALL_COUNT_DOWN_TIME;

    loop {
        let delta_time = get_frame_time();

        clear_background(GRAY);
        match game_state {
            GameState::Title => {
                draw_text("PONG", CENTER_X - 150., CENTER_Y, 150., BLACK);

                quit_button.draw();
                play_button.draw();

                if play_button.mouse_event_listener() {
                    game_state = GameState::BallSpawn;
                    count_down_time = Instant::now();
                }
                if quit_button.mouse_event_listener() {
                    break;
                }
            }
            GameState::BallSpawn => {
                draw_ball_move_count_down(ball_move_count_down);
                if Instant::now() - count_down_time >= Duration::from_secs(1) {
                    ball_move_count_down -= 1;
                    count_down_time = Instant::now();
                    if ball_move_count_down < 0 {
                        ball.set_dir();
                        ball_move_count_down = BALL_COUNT_DOWN_TIME;
                        game_state = GameState::GamePlay;
                    }
                }
            }
            GameState::GamePlay => {
                ball.update(delta_time);
                l_player.update(delta_time);
                r_player.update(delta_time);

                bounce_ball_at_wall(&mut ball, screen_height());
                bounce_ball_on_paddle(&mut ball, l_player.get_paddle());
                bounce_ball_on_paddle(&mut ball, r_player.get_paddle());

                if ball.get_pos().x < l_player.get_paddle().get_x() {
                    r_player.score();
                    if r_player.get_score() == 3 {
                        game_state = GameState::Winner;
                    } else {
                        game_state = GameState::Restart;
                    }
                }
                if ball.get_pos().x > r_player.get_paddle().get_x() + PADDLE_W {
                    l_player.score();
                    if l_player.get_score() == 3 {
                        game_state = GameState::Winner;
                    } else {
                        game_state = GameState::Restart;
                    }
                }
                l_player.draw();
                r_player.draw();
                ball.draw();
            }
            GameState::Winner => {
                if l_player.get_score() == 3 {
                    draw_text(
                        format!("{} WINS!", l_player.get_name()).as_str(),
                        CENTER_X - 250.,
                        CENTER_Y,
                        150.,
                        BLACK,
                    );
                } else {
                    draw_text(
                        format!("{} WINS!", r_player.get_name()).as_str(),
                        CENTER_X - 250.,
                        CENTER_Y,
                        150.,
                        BLACK,
                    );
                }

                play_button.draw();
                quit_button.draw();

                if play_button.mouse_event_listener() {
                    game_state = GameState::Restart;
                }
                if quit_button.mouse_event_listener() {
                    break;
                }
            }
            GameState::Restart => {
                game_state = GameState::BallSpawn;
                restart_game(&mut ball, &mut l_player, &mut r_player);
                count_down_time = Instant::now();
            }
        }
        next_frame().await
    }
}
