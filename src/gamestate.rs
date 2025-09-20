use crate::ball::Ball;
use crate::button::SimpleButton;
use crate::collisions::{bounce_ball_at_wall, bounce_ball_on_paddle};
use crate::game_traits::{Draw, Update};
use crate::player::Player;
use crate::settings::screen::{CENTER_X, CENTER_Y};
use crate::settings::{ball, paddle, screen, ui};
use macroquad::prelude::*;
use std::time::Duration;
use std::time::Instant;

pub enum GameState {
    Title,
    BallSpawn,
    GamePlay,
    Winner,
    Restart,
    Quit,
}

fn restart_game_positions(ball: &mut Ball, l_player: &mut Player, r_player: &mut Player) {
    ball.set_pos(ball::DEFAULT_POSITION);
    ball.set_x_vel(ball::MINIMUM_VELOCITY);
    ball.set_y_vel(ball::MINIMUM_VELOCITY);
    l_player.get_mut_paddle().set_y(paddle::Y_CENTER);
    r_player.get_mut_paddle().set_y(paddle::Y_CENTER);
}

fn draw_ball_move_countdown(count: &i32) {
    draw_text(
        count.to_string().as_str(),
        screen::CENTER_X - (ui::TEXT_SIZE / 2.),
        screen::CENTER_Y,
        ui::TEXT_SIZE,
        ui::COLOR,
    );
}

pub fn title_state(play: &SimpleButton, quit: &SimpleButton) -> Option<GameState> {
    draw_text(
        "PONG",
        screen::CENTER_X - 150.,
        screen::CENTER_Y,
        150.,
        BLACK,
    );

    quit.draw();
    play.draw();

    if play.mouse_event_listener() {
        Some(GameState::BallSpawn)
    } else if quit.mouse_event_listener() {
        Some(GameState::Quit)
    } else {
        None
    }
}

pub fn ball_spawn_state(
    ball: &mut Ball,
    timer: &mut Instant,
    countdown: &mut i32,
) -> Option<GameState> {
    {
        draw_ball_move_countdown(&*countdown);
    }
    if timer.elapsed() >= Duration::from_secs(1) {
        *countdown -= 1;
        *timer = Instant::now();
        if *countdown < 0 {
            ball.set_dir();
            *countdown = ball::SPAWN_TIME;
            Some(GameState::GamePlay)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn play_state(
    ball: &mut Ball,
    l_player: &mut Player,
    r_player: &mut Player,
    dt: f32,
) -> Option<GameState> {
    ball.update(dt);
    l_player.update(dt);
    r_player.update(dt);

    bounce_ball_at_wall(ball, screen_height());
    bounce_ball_on_paddle(ball, l_player.get_paddle());
    bounce_ball_on_paddle(ball, r_player.get_paddle());

    l_player.draw();
    r_player.draw();
    ball.draw();

    if ball.get_pos().x < l_player.get_paddle().get_x() {
        r_player.score();
        if r_player.get_score() == 3 {
            return Some(GameState::Winner);
        } else {
            return Some(GameState::Restart);
        }
    }
    if ball.get_pos().x > r_player.get_paddle().get_x() + paddle::WIDTH {
        l_player.score();
        if l_player.get_score() == 3 {
            return Some(GameState::Winner);
        } else {
            return Some(GameState::Restart);
        }
    }
    return None;
}

pub fn win_state(
    l_player: &Player,
    r_player: &Player,
    play: &SimpleButton,
    quit: &SimpleButton,
) -> Option<GameState> {
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

    play.draw();
    quit.draw();

    if play.mouse_event_listener() {
        return Some(GameState::Restart);
    }
    if quit.mouse_event_listener() {
        return Some(GameState::Quit);
    } else {
        None
    }
}

pub fn restart_state() -> Option<GameState> {
    game_state = GameState::BallSpawn;
    restart_game_positions(&mut ball, &mut l_player, &mut r_player);

    if winner {
        l_player.reset_score();
        r_player.reset_score();
        winner = false;
    }

    count_down_time = Instant::now();
}
