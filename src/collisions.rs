// NEEDED FUNCTIONS:
use crate::ball::*;
use crate::paddle::*;
use crate::pong_types::PADDLE_HEIGHT;

const PADDLE_RANGE_NARROW: f32 = 0.1 * PADDLE_HEIGHT; /* *****|***** */
const PADDLE_RANGE_SLIM: f32 = 0.3 * PADDLE_HEIGHT; /* ****|**|**** */
const PADDLE_RANGE_BROAD: f32 = 0.6 * PADDLE_HEIGHT; /* ***|****|*** */
const PADDLE_RANGE_WIDE: f32 = 1.0 * PADDLE_HEIGHT; /* |**********| */

const VEL_INCR_LVL_ONE: f32 = 5.;
const VEL_INCR_LVL_TWO: f32 = 10.;
const VEL_INCR_LVL_THREE: f32 = 15.;
const VEL_INCR_LVL_FOUR: f32 = 20.;

/// Checks if ball is in range of area of paddle
/// adds variability to ball direction
macro_rules! ball_in_range {
    ($ball:expr, $paddle:expr, $range:expr) => {
        $ball.get_y() > $paddle.get_center_y() - $range
            && $ball.get_y() < $paddle.get_center_y() + $range
    };
}

/// Updates ball velocity when it hits the paddle
macro_rules! update_ball_vel {
    ($ball:expr, $x_vel_incr:expr, $y_vel_incr:expr) => {
        $ball.set_x_vel($ball.get_x_vel() + $x_vel_incr);
        $ball.set_y_vel($ball.get_y_vel() + $y_vel_incr);
    };
}

// func to check if ball is at the wall
pub fn bounce_ball_at_wall(ball: &mut Ball, height: f32) {
    if ball.get_y() - ball.get_radius() <= 0. {
        ball.set_y(ball.get_y() + ball.get_radius() / 2.);
        ball.reverse_dir_y();
    } else if ball.get_y() + ball.get_radius() >= height {
        ball.set_y(ball.get_y() - ball.get_radius() / 2.);
        ball.reverse_dir_y();
    }
}

// func to bounce ball off of paddles
pub fn bounce_ball_on_paddle(ball: &mut Ball, paddle: &Paddle) {
    if ball.get_circle().overlaps_rect(&paddle.get_rect()) {
        if ball.get_y() == paddle.get_y() {
            update_ball_vel!(ball, VEL_INCR_LVL_ONE, VEL_INCR_LVL_ONE);
        } else if ball_in_range!(ball, paddle, PADDLE_RANGE_NARROW) {
            update_ball_vel!(ball, VEL_INCR_LVL_TWO, VEL_INCR_LVL_TWO);
        } else if ball_in_range!(ball, paddle, PADDLE_RANGE_SLIM) {
            update_ball_vel!(ball, VEL_INCR_LVL_THREE, VEL_INCR_LVL_ONE);
        } else if ball_in_range!(ball, paddle, PADDLE_RANGE_BROAD) {
            update_ball_vel!(ball, VEL_INCR_LVL_TWO, VEL_INCR_LVL_THREE);
        } else if ball_in_range!(ball, paddle, PADDLE_RANGE_WIDE) {
            update_ball_vel!(ball, VEL_INCR_LVL_TWO, VEL_INCR_LVL_FOUR);
        }

        ball.reverse_dir_x();
        // must force ball to be outside of the paddle
        // ball.set_x(paddle.get_x() + ball.get_radius() * ball.get_dir_x());
    }
}
