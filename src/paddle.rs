use crate::settings::paddle;
use macroquad::prelude::*;

pub struct Paddle {
    pub dim: Vec2,
    pub pos: Vec2,
    pub vel: f32,
    pub front_x: f32,
    pub color: Color,
}

impl Paddle {
    pub fn new(pos: Vec2, front_x: f32) -> Self {
        Self {
            dim: paddle::DIMENSIONS,
            pos,
            vel: paddle::VELOCITY,
            front_x,
            color: paddle::COLOR,
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

    pub fn update(&mut self, dt: f32, ctrls: (KeyCode, KeyCode)) {
        let distance = self.vel * dt;
        if is_key_down(ctrls.1) {
            self.pos.y += distance;
        }
        if is_key_down(ctrls.0) {
            self.pos.y -= distance;
        }

        self.pos.y = clamp(self.pos.y, 0.0, screen_height() - self.dim.y);
    }

    pub fn draw(&self) {
        draw_rectangle(self.pos.x, self.pos.y, self.dim.x, self.dim.y, self.color);
    }
}
