use crate::{game_traits::*, settings::ui::TEXT_SIZE};
use macroquad::prelude::*;

pub struct SimpleButton {
    text: String,
    dim: Vec2,
    pos: Vec2,
    color: Color,
}

impl SimpleButton {
    pub fn new(text: &str, dim: Vec2, pos: Vec2, color: Color) -> Self {
        Self {
            text: text.to_string(),
            dim,
            pos,
            color,
        }
    }

    pub fn mouse_event_listener(&self) -> bool {
        let (mouse_x, mouse_y) = mouse_position();
        if mouse_x >= self.pos.x
            && mouse_x <= self.pos.x + self.dim.x
            && mouse_y >= self.pos.y
            && mouse_y <= self.pos.y + self.dim.y
        {
            is_mouse_button_pressed(MouseButton::Left)
        } else {
            false
        }
    }
}

impl Draw for SimpleButton {
    fn draw(&self) {
        draw_rectangle(self.pos.x, self.pos.y, self.dim.x, self.dim.y, self.color);
        draw_text(
            self.text.as_str(),
            (self.pos.x + (self.dim.x / 2.)) - ((self.text.len() as f32 / 4.) * TEXT_SIZE as f32),
            (self.pos.y + (self.dim.y)) - (TEXT_SIZE as f32 / 4.),
            TEXT_SIZE as f32,
            BLACK,
        );
    }
}
