use crate::ball::Ball;
use crate::paddle::Paddle;
use crate::settings::paddle;
const PADDLE_HIT_RANGE_NARROW: f32 = 0.1 * paddle::HEIGHT;
const PADDLE_HIT_RANGE_SLIM: f32 = 0.3 * paddle::HEIGHT;
const PADDLE_HIT_RANGE_BROAD: f32 = 0.6 * paddle::HEIGHT;
const PADDLE_HIT_RANGE_WIDE: f32 = 1.0 * paddle::HEIGHT;

const VEL_INCR_SNAIL: f32 = 5.;
const VEL_INCR_SLOW: f32 = 10.;
const VEL_INCR_MEDIUM: f32 = 15.;
const VEL_INCR_FAST: f32 = 20.;

/// using macro to avoid massive if - else tree
macro_rules! ball_paddle_collision {
    ($ball:expr, $paddle:expr, $range:expr, $x_vel_incr:expr, $y_vel_incr:expr) => {
        if $ball.get_pos().y > $paddle.get_center_y() - $range
            && $ball.get_pos().y < $paddle.get_center_y() + $range
        {
            $ball.set_x_vel($ball.get_x_vel() + $x_vel_incr);
            $ball.set_y_vel($ball.get_y_vel() + $y_vel_incr);
        }
    };
}

/// func to check if ball is at the wall
pub fn bounce_ball_at_wall(ball: &mut Ball, height: f32) {
    if ball.get_pos().y - ball.get_radius() < 0. {
        ball.set_y(ball.get_radius());
        ball.reverse_dir_y();
    } else if ball.get_pos().y + ball.get_radius() > height {
        ball.set_y(height - ball.get_radius());
        ball.reverse_dir_y();
    }
}

/// func to bounce ball off of paddles
pub fn bounce_ball_on_paddle(ball: &mut Ball, paddle: &Paddle) {
    if ball.get_circle().overlaps_rect(&paddle.get_rect()) {
        ball_paddle_collision!(
            ball,
            paddle,
            PADDLE_HIT_RANGE_NARROW,
            VEL_INCR_SLOW,
            VEL_INCR_SLOW
        );
        ball_paddle_collision!(
            ball,
            paddle,
            PADDLE_HIT_RANGE_SLIM,
            VEL_INCR_MEDIUM,
            VEL_INCR_SNAIL
        );
        ball_paddle_collision!(
            ball,
            paddle,
            PADDLE_HIT_RANGE_BROAD,
            VEL_INCR_SLOW,
            VEL_INCR_MEDIUM
        );
        ball_paddle_collision!(
            ball,
            paddle,
            PADDLE_HIT_RANGE_WIDE,
            VEL_INCR_SLOW,
            VEL_INCR_FAST
        );
        ball.reverse_dir_x();
        ball.set_x(paddle.get_front_x() + ball.get_radius() * ball.get_dir_x());
    }
}
