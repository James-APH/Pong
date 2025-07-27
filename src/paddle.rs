use crate::pong_types::*;
use crate::settings::*;
use macroquad::prelude::*;

pub struct Paddle {
    dim: (f32, f32),
    pos: Vec2,
    vel: f32,
    ctrl_up: KeyCode,
    ctrl_down: KeyCode,
    color: Color,
}

pub fn init_paddle(pos: Vec2, vel: f32, controls: (KeyCode, KeyCode), color: Color) -> Paddle {
    Paddle {
        dim: (PADDLE_WIDTH, PADDLE_HEIGHT),
        pos,
        vel,
        ctrl_up: controls.0,
        ctrl_down: controls.1,
        color,
    }
}

impl Paddle {
    pub fn get_x(&self) -> f32 {
        self.pos.x
    }

    pub fn get_y(&self) -> f32 {
        self.pos.y
    }

    pub fn set_y(&mut self, y: f32) {
        self.pos.y = y
    }
}

impl Update for Paddle {
    fn update(self: &mut Paddle, dt: f32) {
        let distance = self.vel * dt;
        if is_key_down(self.ctrl_up) {
            self.pos.y += distance;
        }
        if is_key_down(self.ctrl_down) {
            self.pos.y -= distance
        }

        self.pos.y = clamp(self.position.y, 0.0, screen_height() - PADDLE_HEIGHT);
    }
}

impl Draw for Paddle {
    fn draw(self: &Paddle) {
        draw_rectangle(
            self.pos.x,
            self.pos.y,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
            self.color,
        );
    }
}
