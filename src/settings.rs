use macroquad::prelude::*;

// PADDLE RELATED CONSTANTS
pub const PADDLE_HEIGHT: f32 = 128.;
pub const PADDLE_WIDTH: f32 = 32.;
pub const PADDLE_COLOR: Color = WHITE;
pub const PADDLE_VELOCITY: f32 = 250.;
pub const PADDLE_X_OFFSET: f32 = 16.;

// BALL RELATED CONSTANTS
pub const BALL_COLOR: Color = ORANGE;
pub const BALL_RADIUS: f32 = 16.;
pub const MINIMUM_BALL_VELOCITY: f32 = 75.;
pub const MAXIMUM_BALL_VELOCITY: f32 = 500.;

// GAME RELATED CONSTANTS
const WINDOW_WIDTH: i32 = 1000;
const WINDOW_HEIGHT: i32 = 500;

pub fn set_conf() -> Conf {
    Conf {
        window_title: String::from("PONG ULTIMATE BALL OUT"),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}
