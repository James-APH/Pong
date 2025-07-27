use macroquad::prelude::*;

pub const PADDLE_HEIGHT: f32 = 128.;
pub const PADDLE_WIDTH: f32 = 32.;
pub const PADDLE_COLOR: Color = RED;

pub trait Update {
    fn update(&mut self, dt: f32);
}

pub trait Draw {
    fn draw(&self);
}
