use macroquad::{
    prelude::{get_time, vec2, Vec2},
    rand::gen_range,
};

use crate::{
    bounds::Bounds,
    constants::{BALL_RADIUS, BALL_SIZE, BOUNDS, PLAYER_WIDTH},
    physics::{CollisionType, GameObject},
    player::{Player, PlayerPosition},
};

#[derive(Debug)]
pub struct Behavior {
    pub hit_range: (f32, f32),
    pub accuracy: f32,
    pub reaction_time: u16,

    pub hit_position: f32,
    pub collision_time: f64,
    pub predicted_position: Option<Vec2>,
    pub accuracy_variation: f32,
}

impl Behavior {
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

impl Behavior {
    pub fn observe(&mut self, player: &Player, ball: &GameObject) {
        if ball.velocity.length_squared() == 0.0 {
            self.collision_time = get_time();

            self.hit_position = self.hit_position(ball.velocity);

            self.accuracy_variation = self.accuracy_variation();
            
            if ball.velocity.length_squared() == 0.0 {
                println!("set to none");
                self.collision_time = self.reaction_time as f64 / 1000.0;
                // self.predicted_position = None;
            }
        }

        let prediction_position = match player.side {
            PlayerPosition::Left => {
                if ball.velocity.x > 0.0 {
                    return;
                }
                player.object.position.x + PLAYER_WIDTH
            },
            PlayerPosition::Right => {
                if ball.velocity.x < 0.0 {
                    return;
                }
                // println!("{:?}", self.predicted_position);
                player.object.position.x - BALL_RADIUS * 2.0
            },
        };

        self.predicted_position =
            Some(self.predict_ball_position(prediction_position, ball, BOUNDS));
    }

    fn prediction_difficulty(&self, ball_velocity: Vec2) -> f32 {
        if ball_velocity.length_squared() != 0.0 {
            (ball_velocity.y / ball_velocity.x).abs() * (1.0 - self.accuracy)
        } else {
            0.0
        }
    }

    fn hit_position(&self, ball_velocity: Vec2) -> f32 {
        gen_range(
            self.hit_range.0 - self.prediction_difficulty(ball_velocity),
            self.hit_range.1 + self.prediction_difficulty(ball_velocity),
        )
    }

    fn accuracy_variation(&self) -> f32 {
        gen_range(self.accuracy, 2.0 - self.accuracy)
    }

    fn predict_ball_position(&mut self, x: f32, ball: &GameObject, bounds: Bounds) -> Vec2 {
        let height = bounds.h - BALL_SIZE.1;
        let slope = (ball.velocity.y / ball.velocity.x) * self.accuracy_variation;
        let trajectory = -ball.position.x * slope + ball.position.y;

        let y = ((slope * x + trajectory) % (2.0 * height) + 2.0 * height) % (2.0 * height);

        vec2(x, y.min(2.0 * height - y))
    }
}

#[derive(Debug)]
pub struct Ai {
    pub name: &'static str,
    pub behavior: Behavior,
}

impl Ai {
    pub const fn new(
        name: &'static str,
        hit_range: (f32, f32),
        accuracy: f32,
        reaction_time: u16,
    ) -> Self {
        Self {
            name,
            behavior: Behavior::new(hit_range, accuracy, reaction_time),
        }
    }
}
