use macroquad::prelude::*;

use crate::{
    constants::{
        BALL_RADIUS, BOUNDS, BOUNDS_THICKNESS, PLAYER_ACCELERATION, PLAYER_HEIGHT, PLAYER_PADDING,
        PLAYER_VELOCITY, PLAYER_WIDTH,
    },
    physics::{ColliderType, GameObject},
    player::{ControlType, Player, PlayerPosition, UserType, PlayerState}, ai::Ai,
};

pub struct Game {
    players: Vec<PlayerState>,
    ball: GameObject,
    top_bottom_bounds: [GameObject; 2],
    side_bounds: [GameObject; 2],
    camera: Camera2D,
}

impl Game {
    pub fn new() -> Self {
        let mut ball = GameObject::from_pos(
            BOUNDS.center().x - BALL_RADIUS,
            BOUNDS.center().y - BALL_RADIUS,
            ColliderType::Circle(BALL_RADIUS),
        );

        let top_bottom_bounds = [
            GameObject::from_pos(
                BOUNDS.x,
                BOUNDS.y - BOUNDS_THICKNESS,
                ColliderType::Rectangle(BOUNDS.w, BOUNDS_THICKNESS),
            ),
            GameObject::from_pos(
                BOUNDS.x,
                BOUNDS.h,
                ColliderType::Rectangle(BOUNDS.w, BOUNDS_THICKNESS),
            ),
        ];

        let side_bounds = [
            GameObject::from_pos(
                BOUNDS.x - BOUNDS_THICKNESS,
                BOUNDS.y,
                ColliderType::Rectangle(BOUNDS_THICKNESS, BOUNDS.h),
            ),
            GameObject::from_pos(
                BOUNDS.w,
                BOUNDS.y,
                ColliderType::Rectangle(BOUNDS_THICKNESS, BOUNDS.h),
            ),
        ];

        let camera = Camera2D::from_display_rect(Rect::new(
            BOUNDS.x - 1.0,
            BOUNDS.y - 1.0,
            BOUNDS.w + 2.0,
            BOUNDS.h + 2.0,
        ));

        ball.velocity = vec2(-500.0, 10.0);

        Self {
            players: vec![],
            ball,
            top_bottom_bounds,
            side_bounds,
            camera,
        }
    }

    pub fn add_player(&mut self, user_type: UserType, position: PlayerPosition, ai: Ai,) {
        let player_position = match position {
            PlayerPosition::Left => BOUNDS.x + PLAYER_PADDING,
            PlayerPosition::Right => BOUNDS.w - PLAYER_PADDING - PLAYER_WIDTH,
        };

        let player = Player::new(
            "Player 1",
            GameObject::from_pos(
                player_position,
                BOUNDS.center().y - PLAYER_HEIGHT / 2.0,
                ColliderType::Rectangle(PLAYER_WIDTH, PLAYER_HEIGHT),
            ),
            BOUNDS,
            PLAYER_VELOCITY.into(),
            PLAYER_ACCELERATION.into(),
            position,
        );

        let player_state = PlayerState {
            player,
            user_type,
            ai,
        };

        self.players.push(player_state);
    }
}

impl Game {
    pub fn update(&mut self, frame_time: f32) {

        for player_state in self.players.iter_mut() {
            match player_state.user_type {
                UserType::Client(control_type) => {
                    match control_type {
                        ControlType::Mouse => player_state.player.mouse_control(frame_time),
                        ControlType::Keyboard(up, down) => player_state.player.keyboard_control(up, down, frame_time),
                    }
                },
                UserType::Ai => {
                    player_state.ai.behavior.observe(&player_state.player, &self.ball);
                    player_state.player.ai_control(&player_state.ai, frame_time);
                },
            }
            if is_mouse_button_pressed(MouseButton::Left) {
                player_state.switch_user_type();
            }
        }

        let player_objects: Vec<&GameObject> =
            self.players.iter().map(|player_state| &player_state.player.object).collect();

        let bounce_objects: Vec<&GameObject> = self
            .top_bottom_bounds
            .iter()
            .chain(player_objects)
            .collect();

        self.ball.handle_bounces(bounce_objects, frame_time);

        self.update_camera();
    }

    fn update_camera(&mut self) {
        let game_position = BOUNDS.screen_offset();
        let game_size = BOUNDS.screen_size();

        self.camera.viewport = Some((
            game_position.x as i32,
            game_position.y as i32,
            game_size.x as i32,
            game_size.y as i32,
        ));

        set_camera(&self.camera);
    }

    fn show_bounds(&self) {
        for bound in self.top_bottom_bounds.iter() {
            bound.show_object(WHITE);
        }
    }

    fn show_players(&self) {
        for player_state in self.players.iter() {
            player_state.player.object.show_object(WHITE);
        }
    }

    fn show_ball(&self) {
        self.ball.show_object(WHITE);
    }

    pub fn show(&self) {
        self.show_bounds();
        self.show_players();
        self.show_ball();
    }
}
