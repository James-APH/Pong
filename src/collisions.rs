// NEEDED FUNCTIONS:
use crate::ball::*;
use crate::paddle::*;

// bounce ball against top and bottom walls
//fn are_colliding(ball: &mut Ball, paddle: &mut Paddle) {}
// bounce ball against paddles
// need to figure out if

// func to check if ball is at the wall
pub fn bounce_ball_at_wall(ball: &mut Ball, height: f32) {
    if ball.get_y() - ball.get_radius() <= 0. || ball.get_y() + ball.get_radius() >= height {
        ball.reverse_dir_y();
    }
}
// may need to add two special cases for the players (either side in range)
pub fn is_ball_at_paddle(ball: &Ball, paddle: &Paddle) -> bool {
    ball.get_circle().overlaps_rect(&paddle.get_rect())
}

// func to bounce ball off of paddles
pub fn bounce_ball(ball: &mut Ball, paddle: &Paddle) {
    if is_ball_at_paddle(&ball, paddle) {
        ball.reverse_dir_x();
    }
}
