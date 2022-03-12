use macroquad::{prelude::{get_time, vec2, Vec2}, rand::gen_range};

use crate::{
    bounds::Bounds,
    constants::{BALL_RADIUS, BALL_SIZE, BOUNDS, PLAYER_PADDING, PLAYER_WIDTH},
    physics::CollisionType,
};

pub struct AiLogic {
    pub hit_range: (f32, f32),
    pub accuracy: f32,
    pub reaction_time: u16,

    pub hit_position: f32,
    pub collision_time: f64,
    pub predicted_position: Option<Vec2>,
    pub accuracy_variation: f32,
}

impl AiLogic {
    pub const fn new(hit_range: (f32, f32), accuracy: f32, reaction_time: u16) -> Self {
        Self {
            hit_range,
            accuracy,
            reaction_time,

            hit_position: 0.5,
            collision_time: 0.0,
            predicted_position: None,
            accuracy_variation: 1.0,
        }
    }
}

impl AiLogic {
    pub fn observe(
        &mut self,
        player_position: Vec2,
        ball_collisions: Vec<CollisionType>,
        ball_position: Vec2,
        ball_velocity: Vec2,
    ) {
        if !ball_collisions.is_empty() || ball_velocity.length_squared() == 0.0 {
            self.collision_time = get_time();

            self.hit_position = self.hit_position(ball_velocity);

            self.accuracy_variation = self.accuracy_variation();
        }

        if player_position.x == BOUNDS.x + PLAYER_PADDING && ball_velocity.x < 0.0 {
            self.predicted_position = Some(self.predict_ball_position(
                player_position.x + PLAYER_WIDTH,
                ball_position,
                ball_velocity,
                BOUNDS,
            ));
        } else if player_position.x == BOUNDS.w - PLAYER_PADDING - PLAYER_WIDTH
            && ball_velocity.x > 0.0
        {
            self.predicted_position = Some(self.predict_ball_position(
                player_position.x - BALL_RADIUS * 2.0,
                ball_position,
                ball_velocity,
                BOUNDS,
            ));
        } else if ball_velocity.x == 0.0 {
            self.predicted_position = None;
        }
    }

    pub fn prediction_difficulty(&self, ball_velocity: Vec2) -> f32 {
        if ball_velocity.length_squared() != 0.0 {
            (ball_velocity.y / ball_velocity.x).abs() * (1.0 - self.accuracy)
        } else {
            0.0
        }
    }

    pub fn hit_position(&self, ball_velocity: Vec2) -> f32 {
        gen_range(
            self.hit_range.0 - self.prediction_difficulty(ball_velocity),
            self.hit_range.1 + self.prediction_difficulty(ball_velocity)
        )
    }

    fn accuracy_variation(&self) -> f32 {
        gen_range(
            self.accuracy,
            2.0 - self.accuracy
        )
    }

    // TODO: SEE CHANGES IN SLOPE FOR DIFFICULTY
    pub fn predict_ball_position(
        &mut self,
        x: f32,
        ball_position: Vec2,
        ball_velocity: Vec2,
        bounds: Bounds,
    ) -> Vec2 {
        let height = bounds.h - BALL_SIZE.1;
        let slope = (ball_velocity.y / ball_velocity.x) * self.accuracy_variation;
        let trajectory = -ball_position.x * slope + ball_position.y;

        let y = ((slope * x + trajectory) % (2.0 * height) + 2.0 * height) % (2.0 * height);

        vec2(x, y.min(2.0 * height - y))
    }
}

pub struct Ai<'a> {
    pub name: &'a str,
    pub logic: AiLogic,
}

impl<'a> Ai<'a> {
    pub const fn new(
        name: &'a str,
        hit_range: (f32, f32),
        inaccuracy: f32,
        reaction_time: u16,
    ) -> Self {
        Self {
            name,
            logic: AiLogic::new(hit_range, inaccuracy, reaction_time),
        }
    }
}
