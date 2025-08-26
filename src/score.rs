use crate::game_traits::*;
use macroquad::prelude::*;

pub struct Score {
    dim: u16,
    pos: Vec2,
    score: i8,
    color: Color,
}

impl Draw for Score {
    fn draw(&self) {
        draw_text_ex(
            self.score.to_string().as_str(),
            self.pos.x,
            self.pos.y,
            TextParams {
                font_size: self.dim,
                color: self.color,
                ..Default::default()
            },
        );
    }
}

impl Score {
    pub fn new(dim: u16, pos: Vec2, color: Color) -> Self {
        Self {
            dim,
            pos,
            score: 0,
            color,
        }
    }

    pub fn scored(&mut self) {
        self.score += 1;
    }

    pub fn reset(&mut self) {
        self.score = 0;
    }
}
