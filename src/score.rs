use macroquad::{
    prelude::{draw_text, vec2, Vec2, WHITE},
    rand::gen_range,
};

use crate::{
    constants::{BALL_SIZE, BOUNDS},
    physics::GameObject,
    player::PlayerPosition,
};

pub struct Score {
    pub left: usize,
    pub right: usize,
    pub score_time: f32,
    pub start_direction: i32,
}

impl Score {
    pub fn new() -> Self {
        Self {
            left: 0,
            right: 0,
            score_time: 0.2,
            start_direction: match gen_range(0, 2) {
                0 => -1,
                _ => 1,
            },
        }
    }
}

impl Score {
    pub fn show(&self) {
        draw_text(
            &self.left.to_string(),
            (BOUNDS.center().0 + BOUNDS.x) / 2.0,
            BOUNDS.center().1 / 2.0,
            60.0,
            WHITE,
        );
        draw_text(
            &self.right.to_string(),
            (BOUNDS.w + BOUNDS.center().0) / 2.0,
            BOUNDS.center().1 / 2.0,
            60.0,
            WHITE,
        );
    }

    pub fn handle_goals(
        &mut self,
        ball: &mut GameObject,
        goals_left_to_right: &[GameObject; 2],
        game_time: f32,
    ) {
        if let Some(side) = self.check_for_goal(ball, goals_left_to_right) {
            self.score_time = game_time;
            match side {
                PlayerPosition::Left => self.right += 1,
                PlayerPosition::Right => self.left += 1,
            }
        }

        if self.score_time != 0.0 {
            ball.position = Vec2::from(BOUNDS.center()) - Vec2::from(BALL_SIZE) / 2.0;
            ball.velocity = vec2(0.0, 0.0);

            if game_time > self.score_time + 1.0 {
                ball.velocity = vec2(
                    (self.start_direction as f32) * 1000.0,
                    gen_range(-400.0, 400.0),
                );
                self.start_direction *= -1;
                self.score_time = 0.0;
            }
        }
    }

    fn check_for_goal(
        &self,
        ball: &GameObject,
        goals_left_to_right: &[GameObject; 2],
    ) -> Option<PlayerPosition> {
        if ball.check_collisions(&goals_left_to_right[0]).is_some() {
            Some(PlayerPosition::Left)
        } else if ball.check_collisions(&goals_left_to_right[1]).is_some() {
            Some(PlayerPosition::Right)
        } else {
            None
        }
    }
}
