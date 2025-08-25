use macroquad::prelude::*;

pub struct Score {
    dim: Vec2,
    pos: Vec2,
    score: i8,
    color: Color,
}
/*
impl Update for Score {
    //if ...????
}

impl Draw for Score {

}
*/

impl Score {
    fn new(dim: Vec2, pos: Vec2) -> Self {
        Self {
            dim,
            pos,
            score: 0,
            color: WHITE,
        }
    }
}
