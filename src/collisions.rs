use crate::ball::Ball;
use crate::paddle::Paddle;
use crate::settings::paddle;

const PADDLE_HIT_RANGE_NARROW: f32 = 0.1 * paddle::HEIGHT;
const PADDLE_HIT_RANGE_SLIM: f32 = 0.3 * paddle::HEIGHT;
const PADDLE_HIT_RANGE_BROAD: f32 = 0.6 * paddle::HEIGHT;
const PADDLE_HIT_RANGE_WIDE: f32 = 1.0 * paddle::HEIGHT;

const VELOCITY_INCREMENT_SNAIL: f32 = 5.;
const VELOCITY_INCREMENT_SLOW: f32 = 10.;
const VELOCITY_INCREMENT_MEDIUM: f32 = 15.;
const VELOCITY_INCREMENT_FAST: f32 = 20.;

/// Using macro to avoid massive if - else tree
macro_rules! ball_paddle_collision {
    ($ball:expr, $paddle:expr, $range:expr, $x_vel_incr:expr, $y_vel_incr:expr) => {
        if $ball.pos.y > ($paddle.pos.y + $paddle.dim.y / 2.) - $range
            && $ball.pos.y < ($paddle.pos.y + $paddle.dim.y / 2.) + $range
        {
            $ball.vel.x += $x_vel_incr;
            $ball.vel.y += $y_vel_incr;
        }
    };
}

/// Function to check if ball is at the wall
pub fn bounce_ball_at_wall(ball: &mut Ball, height: f32) {
    if ball.pos.y - ball.radius < 0. {
        ball.pos.y = ball.radius;
        ball.dir.y *= -1.;
    } else if ball.pos.y + ball.radius > height {
        ball.pos.y = height - ball.radius;
        ball.dir.y *= -1.;
    }
}

/// Function to bounce ball off of paddles
pub fn bounce_ball_on_paddle(ball: &mut Ball, paddle: &Paddle) {
    if ball.get_circle().overlaps_rect(&paddle.get_rect()) {
        ball_paddle_collision!(
            ball,
            paddle,
            PADDLE_HIT_RANGE_NARROW,
            VELOCITY_INCREMENT_SLOW,
            VELOCITY_INCREMENT_SLOW
        );
        ball_paddle_collision!(
            ball,
            paddle,
            PADDLE_HIT_RANGE_SLIM,
            VELOCITY_INCREMENT_MEDIUM,
            VELOCITY_INCREMENT_SNAIL
        );
        ball_paddle_collision!(
            ball,
            paddle,
            PADDLE_HIT_RANGE_BROAD,
            VELOCITY_INCREMENT_SLOW,
            VELOCITY_INCREMENT_MEDIUM
        );
        ball_paddle_collision!(
            ball,
            paddle,
            PADDLE_HIT_RANGE_WIDE,
            VELOCITY_INCREMENT_SLOW,
            VELOCITY_INCREMENT_FAST
        );
        ball.dir.x *= -1.;
        ball.pos.x = paddle.front_x + ball.radius * ball.dir.x;
    }
}
