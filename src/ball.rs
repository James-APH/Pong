use crate::game_traits::*;
use crate::settings::*;
use macroquad::prelude::*;
use macroquad::rand::rand;

pub struct Ball {
    pos: Vec2,
    vel: Vec2,
    dir: Vec2,
    radius: f32,
    color: Color,
}

impl Ball {
    pub fn new(pos: Vec2, vel: f32, radius: f32, color: Color) -> Self {
        Self {
            pos,
            vel: Vec2::new(vel, vel),
            dir: Vec2::new(0., 0.),
            radius,
            color,
        }
    }

    pub fn get_circle(&self) -> Circle {
        Circle {
            x: self.pos.x,
            y: self.pos.y,
            r: self.radius,
        }
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn get_pos(&self) -> Vec2 {
        self.pos
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    pub fn set_x(&mut self, x: f32) {
        self.pos.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.pos.y = y;
    }

    pub fn get_dir_x(&self) -> f32 {
        self.dir.x
    }

    pub fn set_dir(&mut self) {
        self.dir.x = BALL_DIR_OPS[(rand() % 2) as usize];
        self.dir.y = BALL_DIR_OPS[(rand() % 2) as usize];
    }

    pub fn reverse_dir_y(&mut self) {
        self.dir.y *= -1.
    }

    pub fn reverse_dir_x(&mut self) {
        self.dir.x *= -1.
    }

    pub fn set_x_vel(&mut self, vel: f32) {
        self.vel.x = vel;
    }

    pub fn set_y_vel(&mut self, vel: f32) {
        self.vel.y = vel;
    }

    pub fn get_x_vel(&self) -> f32 {
        self.vel.x
    }

    pub fn get_y_vel(&self) -> f32 {
        self.vel.y
    }
}

impl Update for Ball {
    fn update(self: &mut Ball, dt: f32) {
        let distance_x = self.vel.x * self.dir.x * dt;
        let distance_y = self.vel.y * self.dir.y * dt;
        self.pos.x += distance_x;
        self.pos.y += distance_y;
    }
}

impl Draw for Ball {
    fn draw(self: &Ball) {
        draw_circle(self.pos.x, self.pos.y, self.radius, self.color);
    }
}
