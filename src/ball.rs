use crate::settings::ball;
use macroquad::prelude::*;
use macroquad::rand::rand;

pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
    pub dir: Vec2,
    pub radius: f32,
    pub color: Color,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            pos: ball::DEFAULT_POSITION,
            vel: Vec2::new(ball::MINIMUM_VELOCITY, ball::MINIMUM_VELOCITY),
            dir: Vec2::new(0., 0.),
            radius: ball::RADIUS,
            color: ball::COLOR,
        }
    }

    pub fn get_circle(&self) -> Circle {
        Circle {
            x: self.pos.x,
            y: self.pos.y,
            r: self.radius,
        }
    }

    pub fn initialize_dirrection(&mut self) {
        self.dir.x = ball::DIRECTION_OPS[(rand() % 2) as usize];
        self.dir.y = ball::DIRECTION_OPS[(rand() % 2) as usize];
    }

    pub fn update(&mut self, dt: f32) {
        let distance_x = self.vel.x * self.dir.x * dt;
        let distance_y = self.vel.y * self.dir.y * dt;
        self.pos.x += distance_x;
        self.pos.y += distance_y;
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, self.color);
    }
}
