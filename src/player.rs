use std::f32::INFINITY;

use macroquad::prelude::{
    get_time, is_key_down, is_mouse_button_pressed, mouse_position, touches, vec2, KeyCode,
    MouseButton, Vec2,
};

use crate::{
    ai::{Ai, Behavior},
    bounds::Bounds,
    constants::{BALL_SIZE, PLAYER_HEIGHT, PLAYER_WIDTH},
    physics::GameObject,
};

#[derive(Debug, Clone, Copy)]
pub enum ControlType {
    Mouse,
    Keyboard(KeyCode, KeyCode),
}

#[derive(Debug, Clone, Copy)]
pub enum UserType {
    Client(ControlType),
    Ai(Ai),
}


pub struct PlayerState {
    pub player: Player,
    pub user_type: UserType,
    pub ai: Ai,
}

impl PlayerState {
    // pub fn switch_user_type(&mut self) {
    //     self.user_type = match self.user_type {
    //         UserType::Client(_) => UserType::Ai,
    //         UserType::Ai => UserType::Ai,
    //     }
    // }

    pub fn handle_state(&mut self, ball: &GameObject, frame_time: f32) {
        match self.user_type {
            UserType::Client(control_type) => {
                match control_type {
                    ControlType::Mouse => self.player.mouse_control(frame_time),
                    ControlType::Keyboard(up, down) => self.player.keyboard_control(up, down, frame_time),
                }
            },
            UserType::Ai(_) => {
                self.ai.behavior.observe(&self.player, ball);
                self.player.ai_control(&self.ai, frame_time);
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerPosition {
    Left,
    Right,
}

pub struct Player {
    pub name: String,
    pub object: GameObject,
    pub bounds: Bounds,
    pub max_velocity: Vec2,
    pub max_acceleration: Vec2,
    pub side: PlayerPosition,
}

impl Player {
    pub fn new(
        name: &str,
        mut object: GameObject,
        bounds: Bounds,
        max_velocity: Vec2,
        max_acceleration: Vec2,
        side: PlayerPosition,
    ) -> Self {
        object.is_player = true;
        Self {
            name: name.to_owned(),
            object,
            bounds,
            max_velocity,
            max_acceleration,
            side,
        }
    }
}

impl Player {

    pub fn keyboard_control(&mut self, up: KeyCode, down: KeyCode, frame_time: f32) {
        let towards;
        if !(is_key_down(up) ^ is_key_down(down)) {
            towards = self.object.position;
        } else {
            if is_key_down(up) {
                towards = vec2(self.object.position.x, -INFINITY);
            } else {
                towards = vec2(self.object.position.x, INFINITY);
            }
        }
        self.object.move_towards_in_bounds(
            towards,
            self.max_velocity * 0.5,
            self.max_acceleration * 0.6,
            self.bounds,
            frame_time,
        );
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
            self.max_acceleration,
            self.bounds,
            frame_time,
        );
    }

    pub fn ai_control(&mut self, ai: &Ai, frame_time: f32) {
        self.name = ai.name.to_owned();
        if let Some(predicted_position) = ai.behavior.predicted_position {
            if get_time() - ai.behavior.collision_time >= ai.behavior.reaction_time as f64 / 1000.0 {
                let adjusted_prediction = Vec2::from(BALL_SIZE) / 2.0 + predicted_position
                    - self.object.collider.rect.size() * ai.behavior.hit_position;

                self.object.move_towards_in_bounds(
                    adjusted_prediction,
                    self.max_velocity,
                    self.max_acceleration,
                    self.bounds,
                    frame_time,
                );
            }
        } else {
            self.object.move_towards_in_bounds(
                self.bounds.center() - Vec2::from((PLAYER_WIDTH, PLAYER_HEIGHT)) / 2.0,
                self.max_velocity / 3.0,
                self.max_acceleration,
                self.bounds,
                frame_time,
            );
        }
    }
}
