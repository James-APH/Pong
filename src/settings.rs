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
pub const BALL_COUNT_DOWN_TIME: i32 = 3;

// GAME RELATED CONSTANTS
pub const SCREEN_W: f32 = 1000.;
pub const SCREEN_H: f32 = 500.;

pub const TEXT_SIZE: u16 = 50;
pub const CENTER_X: f32 = SCREEN_W / 2.0;
pub const CENTER_Y: f32 = SCREEN_H / 2.0;

pub const PADDLE_CENTER: f32 = CENTER_Y - PADDLE_H / 2.0;
pub const PADDLE_DIM: Vec2 = Vec2::new(PADDLE_W, PADDLE_H);

pub const PADDLE_POS_L: Vec2 = Vec2::new(PADDLE_X_OFFSET, PADDLE_CENTER);
pub const PADDLE_POS_R: Vec2 = Vec2::new(SCREEN_W - PADDLE_W - PADDLE_X_OFFSET, PADDLE_CENTER);

pub const SCORE_POS_L: Vec2 = Vec2::new(TEXT_SIZE as f32 * 2., TEXT_SIZE as f32 * 2.);
pub const SCORE_POS_R: Vec2 = Vec2::new(SCREEN_W - TEXT_SIZE as f32 * 2., TEXT_SIZE as f32 * 2.);

pub const DEFAULT_BALL_POS: Vec2 = Vec2::new(CENTER_X, CENTER_Y);
pub const BALL_DIR_OPS: [f32; 2] = [-1., 1.];

// BUTTON RELATED CONSTANTS
pub const BUTTON_DIM: Vec2 = Vec2::new(200., 50.);
pub const BUTTON_Y: f32 = 300.;

pub fn set_conf() -> Conf {
    Conf {
        window_title: String::from("PONG ULTIMATE BALL OUT"),
        window_width: SCREEN_W as i32,
        window_height: SCREEN_H as i32,
        ..Default::default()
    }
}
