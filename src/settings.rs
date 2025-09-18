use macroquad::prelude::*;

// PADDLE RELATED CONSTANTS
pub mod screen {
    use super::*;
    pub const WIDTH: f32 = 1000.;
    pub const HEIGHT: f32 = 500.;
    pub const CENTER_X: f32 = WIDTH / 2.0;
    pub const CENTER_Y: f32 = HEIGHT / 2.0;
}

pub mod paddle {
    use super::*;
    pub const HEIGHT: f32 = 128.;
    pub const WIDTH: f32 = 32.;
    pub const COLOR: Color = WHITE;
    pub const VELOCITY: f32 = 250.;
    pub const X_OFFSET: f32 = 16.;
    pub const Y_CENTER: f32 = screen::CENTER_Y - HEIGHT / 2.0;
    pub const DIMENSIONS: Vec2 = Vec2::new(WIDTH, HEIGHT);
    pub const POSITION_LEFT: Vec2 = Vec2::new(X_OFFSET, Y_CENTER);
    pub const POSITION_RIGHT: Vec2 = Vec2::new(screen::WIDTH - WIDTH - X_OFFSET, Y_CENTER);
}

pub mod ball {
    use super::*;
    pub const COLOR: Color = ORANGE;
    pub const RADIUS: f32 = 16.;
    pub const MINIMUM_VELOCITY: f32 = 75.;
    pub const SPAWN_TIME: i32 = 3;
    pub const DEFAULT_POSITION: Vec2 = Vec2::new(screen::CENTER_X, screen::CENTER_Y);
    pub const DIRECTION_OPS: [f32; 2] = [-1., 1.];
}

pub mod score {
    use super::*;
    pub const POSITION_LEFT: Vec2 = Vec2::new(ui::TEXT_SIZE as f32 * 2., ui::TEXT_SIZE as f32 * 2.);
    pub const POSITION_RIGHT: Vec2 = Vec2::new(
        screen::WIDTH - ui::TEXT_SIZE as f32 * 2.,
        ui::TEXT_SIZE as f32 * 2.,
    );
    pub const COLOR: Color = RED;
}

pub mod ui {
    use super::*;
    pub const TEXT_SIZE: u16 = 50;
}

// BUTTON RELATED CONSTANTS
pub const BUTTON_DIM: Vec2 = Vec2::new(200., 50.);
pub const BUTTON_Y: f32 = 300.;

pub fn set_conf() -> Conf {
    Conf {
        window_title: String::from("PONG ULTIMATE BALL OUT"),
        window_width: screen::WIDTH as i32,
        window_height: screen::HEIGHT as i32,
        ..Default::default()
    }
}
