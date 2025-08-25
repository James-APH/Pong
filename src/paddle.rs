use crate::game_traits::*;
use crate::settings::*;
use macroquad::prelude::*;

pub struct Paddle {
    dim: Vec2,
    pos: Vec2,
    vel: f32,
    front_x: f32,
    color: Color,
}

impl Paddle {
    pub fn new(dim: Vec2, pos: Vec2, vel: f32, front_x: f32) -> Self {
        Self {
            dim,
            pos,
            vel,
            front_x,
            color: PADDLE_COLOR,
        }
    }

    pub fn get_rect(&self) -> Rect {
        Rect {
            x: self.pos.x,
            y: self.pos.y,
            w: self.dim.x,
            h: self.dim.y,
        }
    }

    pub fn get_y(&self) -> f32 {
        self.pos.y
    }

    pub fn get_center_y(&self) -> f32 {
        self.pos.y + PADDLE_HEIGHT / 2.
    }

    pub fn set_y(&mut self, y: f32) {
        self.pos.y = y
    }

    pub fn get_front_x(&self) -> f32 {
        self.front_x
    }

    pub fn update(&mut self, dt: f32, ctrls: (KeyCode, KeyCode)) {
        let distance = self.vel * dt;
        if is_key_down(ctrls.1) {
            self.pos.y += distance;
        }
        if is_key_down(ctrls.0) {
            self.pos.y -= distance
        }

        self.pos.y = clamp(self.pos.y, 0.0, screen_height() - PADDLE_HEIGHT);
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
