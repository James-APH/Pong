use crate::game_traits::*;
use crate::paddle::*;
use crate::score::*;
use crate::settings::*;
use macroquad::input::KeyCode;
use macroquad::prelude::*;

pub struct Player {
    name: String,
    paddle: Paddle,
    score: Score,
    ctrls: (KeyCode, KeyCode),
}

impl Player {
    pub fn new(name: &str, paddle: Paddle, score: Score, ctrls: (KeyCode, KeyCode)) -> Self {
        Self {
            name: String::from(name),
            paddle,
            score,
            ctrls,
        }
    }

    pub fn get_paddle(&self) -> &Paddle {
        &self.paddle
    }
}

impl Update for Player {
    fn update(&mut self, dt: f32) {
        self.paddle.update(dt, self.ctrls);
    }
}

impl Draw for Player {
    fn draw(&self) {
        self.paddle.draw();
        self.score.draw();
    }
}
