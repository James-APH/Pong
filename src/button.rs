use crate::game_traits::*;
use macroquad::prelude::*;

pub struct Button {
    text: String,
    dim: Vec2,
    pos: Vec2,
    pressed: bool,
    color: Color,
}

impl Button {
    pub fn new(text: &str, dim: Vec2, pos: Vec2, color: Color) -> Self {
        Self {
            text: text.to_string(),
            dim,
            pos,
            pressed: false,
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

impl Update for Button {
    fn update(&mut self, dt: f32) {}
}

impl Draw for Button {
    fn draw(&self) {
        draw_rectangle(self.pos.x, self.pos.y, self.dim.x, self.dim.y, self.color);
    }
}
