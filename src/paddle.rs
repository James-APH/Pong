use crate::pong_types::*;
use macroquad::prelude::*;

pub struct Paddle {
    width: f32,
    height: f32,
    x: f32,
    y: f32,
    speed: f32,
    ctrl_up: KeyCode,
    ctrl_down: KeyCode,
    color: Color,
}

pub fn init_paddle(
    dimensions: (f32, f32),
    coords: (f32, f32),
    speed: f32,
    controls: (KeyCode, KeyCode),
    color: Color,
) -> Paddle {
    Paddle {
        width: dimensions.0,
        height: dimensions.1,
        x: coords.0,
        y: coords.1,
        speed,
        ctrl_up: controls.0,
        ctrl_down: controls.1,
        color,
    }
}

impl Paddle {
    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y
    }
}

impl Update for Paddle {
    fn update(self: &mut Paddle, dt: f32) {
        let distance = self.speed * dt;
        if is_key_down(self.ctrl_up) {
            self.y += distance;
        }
        if is_key_down(self.ctrl_down) {
            self.y -= distance
        }

        self.y = clamp(self.y, 0.0, screen_height() - self.height);
    }
}

impl Draw for Paddle {
    fn draw(self: &Paddle) {
        draw_rectangle(self.x, self.y, self.width, self.height, self.color);
    }
}
