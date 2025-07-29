// NEEDED FUNCTIONS:
use crate::ball::*;
use crate::paddle::*;

// bounce ball against top and bottom walls
//fn are_colliding(ball: &mut Ball, paddle: &mut Paddle) {}
// bounce ball against paddles
// need to figure out if

pub fn bounce_ball_on_wall(ball: &mut Ball, height: f32) {
    if ball.get_y() - ball.get_radius() <= 0. || ball.get_y() + ball.get_radius() >= height {
        ball.reverse_dir_y();
    }
}

pub fn is_ball_at_paddle(ball: &Ball, paddle: &paddle) -> bool {
    ball.get_y()
}
