pub trait Update {
    fn update(&mut self, dt: f32);
}

pub trait Draw {
    fn draw(&self);
}
