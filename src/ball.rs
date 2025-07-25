use crate::pong_types::*;
use macroquad::prelude::*;

pub struct Ball {
    radius: f32,
    coords: (f32, f32),
    speed: (f32, f32),
    color: Color,
}

pub fn init_ball(radius: f32, coords: (f32, f32), speed: (f32, f32), color: Color) -> Ball {
    Ball {
        radius,
        coords,
        speed,
        color,
    }
}

impl Ball {
    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn get_y(&self) -> f32 {
        self.coords.0
    }

    pub fn get_x(&self) -> f32 {
        self.coords.1
    }

    pub fn set_y(&mut self, y: f32) {
        self.coords.1 = y
    }
}

impl Update for Ball {
    fn update(self: &mut Ball, dt: f32) {
        let distance_x = self.speed.0 * dt;
        let distance_y = self.speed.1 * dt;

        // Will need to think about this one
        // perhaps start the ballout in a random
        // direction.
        //
        // Will mean I need to have 2 different speeds, one for x and one for y
    }
}

impl Draw for Ball {
    fn draw(self: &Ball) {
        draw_circle(self.coords.0, self.coords.1, self.radius, self.color);
    }
}
