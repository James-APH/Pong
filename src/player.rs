use crate::game_traits::{Draw, Update};
use crate::paddle::Paddle;
use crate::score::Score;
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

    pub fn get_mut_paddle(&mut self) -> &mut Paddle {
        &mut self.paddle
    }

    pub fn score(&mut self) {
        self.score.scored();
    }

    pub fn get_score(&self) -> i8 {
        self.score.get_current_score()
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_score(&mut self) {
        self.score.reset();
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
