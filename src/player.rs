use crate::game_traits::*;
use crate::paddle::*;
use crate::settings::*;
use macroquad::prelude::*;
use macroquad::{input::KeyCode, window::screen_height};

pub struct Player {
    name: String,
    paddle: Paddle,
    score: i32,
    ctrls: (KeyCode, KeyCode),
}

impl Player {
    pub fn new(name: &str, paddle: Paddle, ctrls: (KeyCode, KeyCode)) -> Self {
        Self {
            name: String::from(name),
            paddle,
            score: 0,
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
    }
}

fn init_player(name: &str, paddle: Paddle, ctrls: (KeyCode, KeyCode)) -> Player {
    Player {
        name: String::from(name),
        paddle,
        score: 0,
        ctrls,
    }
}
