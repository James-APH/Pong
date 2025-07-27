use macroquad::prelude::*;

// PADDLE RELATED CONSTANTS
pub const PADDLE_HEIGHT: f32 = 128.;
pub const PADDLE_WIDTH: f32 = 32.;
pub const PADDLE_COLOR: Color = RED;

// BALL RELATED CONSTANTS
pub const BALL_COLOR: Color = ORANGE;
pub const MAX_BALL_VEL: f32 = 600.;
pub const BALL_RADIUS: f32 = 16.;

pub trait Update {
    fn update(&mut self, dt: f32);
}

pub trait Draw {
    fn draw(&self);
}
