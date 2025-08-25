use macroquad::prelude::*;

// PADDLE RELATED CONSTANTS
pub const PADDLE_H: f32 = 128.;
pub const PADDLE_W: f32 = 32.;
pub const PADDLE_COLOR: Color = WHITE;
pub const PADDLE_VEL: f32 = 250.;
pub const PADDLE_X_OFFSET: f32 = 16.;

// BALL RELATED CONSTANTS
pub const BALL_COLOR: Color = ORANGE;
pub const BALL_RADIUS: f32 = 16.;
pub const MIN_BALL_VEL: f32 = 75.;
pub const MAX_BALL_VEL: f32 = 500.;

// GAME RELATED CONSTANTS
pub const SCREEN_W: i32 = 1000;
pub const SCREEN_H: i32 = 500;

pub fn set_conf() -> Conf {
    Conf {
        window_title: String::from("PONG ULTIMATE BALL OUT"),
        window_width: SCREEN_W,
        window_height: SCREEN_H,
        ..Default::default()
    }
}
