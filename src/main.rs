mod ball;
mod button;
mod collisions;
mod gamestate;
mod paddle;
mod player;
mod score;
mod settings;

use crate::ball::Ball;
use crate::button::SimpleButton;
use crate::paddle::Paddle;
use crate::player::Player;
use crate::score::Score;
use crate::settings::{BUTTON_DIM, BUTTON_Y, set_conf};
use macroquad::prelude::*;
use std::time::Instant;

#[macroquad::main(set_conf)]
async fn main() {
    let l_paddle = Paddle::new(
        settings::paddle::POSITION_LEFT,
        settings::paddle::POSITION_LEFT.x + settings::paddle::WIDTH,
    );
    let r_paddle = Paddle::new(
        settings::paddle::POSITION_RIGHT,
        settings::paddle::POSITION_RIGHT.x,
    );

    let l_score = Score::new(
        settings::ui::TEXT_SIZE,
        settings::score::POSITION_LEFT,
        settings::score::COLOR,
    );
    let r_score = Score::new(
        settings::ui::TEXT_SIZE,
        settings::score::POSITION_RIGHT,
        settings::score::COLOR,
    );
    let mut l_player = Player::new("LEFT", l_paddle, l_score, (KeyCode::W, KeyCode::S));
    let mut r_player = Player::new("RIGHT", r_paddle, r_score, (KeyCode::Up, KeyCode::Down));
    let mut ball = Ball::new();
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
            x: settings::screen::WIDTH - 100. - BUTTON_DIM.x,
            y: BUTTON_Y,
        },
        RED,
    );

    // GAME RELATED VARS
    let mut state = gamestate::GameState::Title;

    let mut timer = Instant::now();
    let mut countdown: i32 = settings::ball::SPAWN_TIME;
    loop {
        let delta_time = get_frame_time();

        clear_background(GRAY);
        match state {
            gamestate::GameState::Title => {
                state = match gamestate::title_state(&play_button, &quit_button) {
                    Some(g_state) => g_state,
                    None => state,
                };
                timer = Instant::now();
            }
            gamestate::GameState::BallSpawn => {
                state = match gamestate::ball_spawn_state(&mut ball, &mut timer, &mut countdown) {
                    Some(g_state) => g_state,
                    None => state,
                }
            }
            gamestate::GameState::GamePlay => {
                state =
                    match gamestate::play_state(&mut ball, &mut l_player, &mut r_player, delta_time)
                    {
                        Some(g_state) => g_state,
                        None => state,
                    }
            }
            gamestate::GameState::Winner => {
                state = match gamestate::win_state(&l_player, &r_player, &play_button, &quit_button)
                {
                    Some(g_state) => g_state,
                    None => state,
                }
            }
            gamestate::GameState::Restart => {
                state =
                    gamestate::restart_state(&mut ball, &mut l_player, &mut r_player, &mut timer);
            }
            gamestate::GameState::Quit => break,
        }
        next_frame().await;
    }
}
