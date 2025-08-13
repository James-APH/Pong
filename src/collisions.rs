// NEEDED FUNCTIONS:
use crate::ball::*;
use crate::paddle::*;
use crate::pong_types::PADDLE_HEIGHT;

const PADDLE_RANGE_NARROW: f32 = 0.1 * PADDLE_HEIGHT; /* *****|***** */
const PADDLE_RANGE_SLIM: f32 = 0.3 * PADDLE_HEIGHT; /* ****|**|**** */
const PADDLE_RANGE_BROAD: f32 = 0.6 * PADDLE_HEIGHT; /* ***|****|*** */
const PADDLE_RANGE_WIDE: f32 = 1.0 * PADDLE_HEIGHT; /* |**********| */

const SPEED_INCREASE_LEVEL_ONE: f32 = 5.;
const SPEED_INCREASE_LEVEL_TWO: f32 = 10.;
const SPEED_INCREASE_LEVEL_THREE: f32 = 15.;
const SPEED_INCREASE_LEVEL_FOUR: f32 = 20.;
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
    if is_ball_at_paddle(ball, paddle) {
        if ball.get_y() == paddle.get_y() {
            ball.reverse_dir_x();
        } else if ball.get_y() > paddle.get_center_y() - PADDLE_RANGE_NARROW
            && ball.get_y() < paddle.get_center_y() + PADDLE_RANGE_NARROW
        {
            ball.set_x_vel(ball.get_x_vel() + SPEED_INCREASE_LEVEL_ONE);
            ball.set_y_vel(ball.get_y_vel() + SPEED_INCREASE_LEVEL_ONE);
            ball.reverse_dir_x();
        } else if ball.get_y() > paddle.get_center_y() - PADDLE_RANGE_SLIM
            && ball.get_y() < paddle.get_center_y() + PADDLE_RANGE_SLIM
        {
            ball.set_x_vel(ball.get_x_vel() + SPEED_INCREASE_LEVEL_THREE);
            ball.set_y_vel(ball.get_y_vel() + SPEED_INCREASE_LEVEL_ONE);
            ball.reverse_dir_x();
        } else if ball.get_y() > paddle.get_center_y() - PADDLE_RANGE_BROAD
            && ball.get_y() < paddle.get_center_y() + PADDLE_RANGE_BROAD
        {
            ball.set_x_vel(ball.get_x_vel() + SPEED_INCREASE_LEVEL_TWO);
            ball.set_y_vel(ball.get_y_vel() + SPEED_INCREASE_LEVEL_THREE);
            ball.reverse_dir_x();
        } else if ball.get_y() > paddle.get_center_y() - PADDLE_RANGE_WIDE
            && ball.get_y() < paddle.get_center_y() + PADDLE_RANGE_WIDE
        {
            ball.set_x_vel(ball.get_x_vel() + SPEED_INCREASE_LEVEL_TWO);
            ball.set_y_vel(ball.get_y_vel() + SPEED_INCREASE_LEVEL_FOUR);
            ball.reverse_dir_x();
        }
    }
}
