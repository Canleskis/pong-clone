use std::f32::INFINITY;

use macroquad::prelude::{get_time, is_key_down, mouse_position, touches, vec2, KeyCode, Vec2};

use crate::{
    ai::Ai,
    bounds::Bounds,
    constants::{BALL_SIZE, PLAYER_HEIGHT, PLAYER_WIDTH},
    physics::GameObject,
};

pub struct Player {
    pub name: String,
    pub object: GameObject,
    pub score: u8,
    pub bounds: Bounds,
    pub max_velocity: Vec2,
}

impl Player {
    pub fn new(name: &str, mut object: GameObject, bounds: Bounds, max_velocity: Vec2) -> Self {
        object.is_player = true;
        Self {
            name: name.to_owned(),
            object,
            score: 0,
            bounds,
            max_velocity,
        }
    }
}

impl Player {
    pub fn scored(&mut self) {
        self.score += 1;
    }

    pub fn keyboard_control(&mut self, up: KeyCode, down: KeyCode, frame_time: f32) {
        if !(is_key_down(up) ^ is_key_down(down)) {
            self.object.velocity = (0.0, 0.0).into();
            return;
        }
        if is_key_down(up) {
            self.object.move_towards_in_bounds(
                vec2(self.object.position.y, -INFINITY),
                self.max_velocity * 0.5,
                0,
                self.bounds,
                frame_time,
            );
        }
        if is_key_down(down) {
            self.object.move_towards_in_bounds(
                vec2(self.object.position.y, INFINITY),
                self.max_velocity * 0.5,
                0,
                self.bounds,
                frame_time,
            );
        }
    }

    pub fn mouse_control(&mut self, frame_time: f32) {
        let mut mouse_position_bounds = self.bounds.convert_to_local(mouse_position().into())
            - self.object.collider.rect.size() / 2.0;
        if let Some(touch) = touches().last() {
            mouse_position_bounds = self.bounds.convert_to_local(touch.position)
                - self.object.collider.rect.size() / 2.0;
        }
        self.object.move_towards_in_bounds(
            mouse_position_bounds,
            self.max_velocity,
            0,
            self.bounds,
            frame_time,
        );
    }

    pub fn ai_control(&mut self, ai: &Ai, frame_time: f32) {
        self.name = ai.name.to_owned();
        if let Some(predicted_position) = ai.logic.predicted_position {
            if get_time() - ai.logic.collision_time >= ai.logic.reaction_time as f64 / 1000.0 {
                let adjusted_prediction = Vec2::from(BALL_SIZE) / 2.0 + predicted_position
                    - self.object.collider.rect.size() * ai.logic.hit_position;
                self.object.move_towards_in_bounds(
                    adjusted_prediction,
                    self.max_velocity,
                    8,
                    self.bounds,
                    frame_time,
                );
            }
        } else {
            self.object.move_towards_in_bounds(
                self.bounds.center() - Vec2::from((PLAYER_WIDTH, PLAYER_HEIGHT)) / 2.0,
                self.max_velocity / 3.0,
                16,
                self.bounds,
                frame_time,
            );
        }
    }
}
