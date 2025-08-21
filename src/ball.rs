use crate::pong_types::*;
use macroquad::prelude::*;

pub struct Ball {
    pos: Vec2,
    vel: Vec2,
    dir: Vec2,
    radius: f32,
    color: Color,
}

pub fn init_ball(pos: Vec2, vel: Vec2, dir: Vec2) -> Ball {
    Ball {
        pos,
        vel,
        dir,
        radius: BALL_RADIUS,
        color: BALL_COLOR,
    }
}

impl Ball {
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

    pub fn get_y(&self) -> f32 {
        self.pos.y
    }

    pub fn get_x(&self) -> f32 {
        self.pos.x
    }

    pub fn get_dir_x(&self) -> f32 {
        self.dir.x
    }

    pub fn set_y(&mut self, y: f32) {
        self.pos.y = y;
    }

    pub fn set_x(&mut self, x: f32) {
        self.pos.x = x;
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

    pub fn get_slope(&self) -> f32 {
        let y_0 = self.pos.y;
        let x_0 = self.pos.x;
        let y_1 = self.pos.y + (self.vel.y * self.dir.y);
        let x_1 = self.pos.x + (self.vel.x * self.dir.x);
        (y_1 - y_0) / (x_1 - x_0)
    }

    pub fn get_y_intersect(&self) -> f32 {
        let y_0 = self.pos.y;
        let x_0 = self.pos.x;
        y_0 - (self.get_slope() * x_0)
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
