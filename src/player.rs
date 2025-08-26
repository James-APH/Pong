use crate::game_traits::*;
use crate::paddle::*;
use crate::score::*;
use macroquad::input::KeyCode;
use macroquad::prelude::*;

pub struct Player {
    paddle: Paddle,
    score: Score,
    ctrls: (KeyCode, KeyCode),
}

impl Player {
    pub fn new(paddle: Paddle, score: Score, ctrls: (KeyCode, KeyCode)) -> Self {
        Self {
            paddle,
            score,
            ctrls,
        }
    }

    pub fn get_paddle(&self) -> &Paddle {
        &self.paddle
    }

    pub fn get_mut_paddle(&mut self) -> &mut Paddle {
        &mut self.paddle
    }

    pub fn score(&mut self) {
        self.score.scored();
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
