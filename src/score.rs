use crate::game_traits::Draw;
use macroquad::prelude::*;

pub struct Score {
    dim: f32,
    pos: Vec2,
    count: i8,
    color: Color,
}

impl Draw for Score {
    fn draw(&self) {
        draw_text(
            self.count.to_string().as_str(),
            self.pos.x,
            self.pos.y,
            self.dim,
            self.color,
        );
    }
}

impl Score {
    pub fn new(dim: f32, pos: Vec2, color: Color) -> Self {
        Self {
            dim,
            pos,
            count: 0,
            color,
        }
    }

    pub fn scored(&mut self) {
        self.count += 1;
    }

    pub fn get_current_score(&self) -> i8 {
        self.count
    }

    pub fn reset(&mut self) {
        self.count = 0;
    }
}
