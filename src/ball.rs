use crate::pong_types::*;
use macroquad::prelude::*;

pub struct Ball {
    pos: Vec2,
    vel: Vec2,
    dir: Vec2,
    radius: f32,
    max_vel: f32,
    color: Color,
}

pub fn init_ball(pos: Vec2, vel: Vec2, dir: Vec2) -> Ball {
    Ball {
        pos,
        vel,
        dir,
        radius: BALL_RADIUS,
        max_vel: MAX_BALL_VEL,
        color: BALL_COLOR,
    }
}

impl Ball {
    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn get_y(&self) -> f32 {
        self.pos.y
    }

    pub fn get_x(&self) -> f32 {
        self.pos.x
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
}

impl Update for Ball {
    fn update(self: &mut Ball, dt: f32) {
        let distance_x = self.vel.x * self.dir.x * dt;
        let distance_y = self.vel.y * self.dir.y * dt;
        self.pos.x += distance_x;
        self.pos.y += distance_y;
        // Will need to think about this one
        // perhaps start the ballout in a random
        // direction.
        //
        // Will mean I need to have 2 different speeds, one for x and one for y
    }
}

impl Draw for Ball {
    fn draw(self: &Ball) {
        draw_circle(self.pos.x, self.pos.y, self.radius, self.color);
    }
}
