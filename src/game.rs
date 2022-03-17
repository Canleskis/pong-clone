use macroquad::{prelude::*, rand::gen_range};

use crate::{
    constants::{
        BALL_RADIUS, BOUNDS, BOUNDS_THICKNESS, PLAYER_ACCELERATION, PLAYER_HEIGHT, PLAYER_PADDING,
        PLAYER_VELOCITY, PLAYER_WIDTH, BALL_SIZE, PREDICT,
    },
    physics::{ColliderType, GameObject},
    player::{Player, PlayerPosition, UserType, PlayerState, ControlType},
};

pub struct Score {
    pub left: usize,
    pub right: usize,
    pub score_time: f64,
    pub start_direction: i32,
}

impl Score {
    fn new() -> Self {
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
            (BOUNDS.center().x + BOUNDS.x) / 2.0,
            BOUNDS.center().y / 2.0,
            60.0,
            WHITE,
        );
        draw_text(
            &self.right.to_string(),
            (BOUNDS.w + BOUNDS.center().x) / 2.0,
            BOUNDS.center().y / 2.0,
            60.0,
            WHITE,
        );
    }

    pub fn handle_goals(&mut self, ball: &mut GameObject, goals_left_to_right: &[GameObject; 2]) {
        if let Some(side) = self.check_for_goal(ball, goals_left_to_right) {
            self.score_time = get_time();
            match side {
                PlayerPosition::Left => self.left += 1,
                PlayerPosition::Right => self.right += 1,
            }
        }

        if self.score_time != 0.0 {
            ball.position = BOUNDS.center() - Vec2::from(BALL_SIZE) / 2.0;
            ball.velocity = vec2(0.0, 0.0);

            if get_time() > self.score_time + 1.0 {
                ball.velocity = vec2((self.start_direction as f32) * 1000.0, gen_range(-400.0, 400.0));
                self.start_direction *= -1;
                self.score_time = 0.0;
            }
        }
    }

    fn check_for_goal(&self, ball: &GameObject, goals_left_to_right: &[GameObject; 2]) -> Option<PlayerPosition> {
        if ball.check_collisions(&goals_left_to_right[0]).is_some() {
            Some(PlayerPosition::Left)
        } else if ball.check_collisions(&goals_left_to_right[1]).is_some() {
            Some(PlayerPosition::Right)
        } else {
            None
        }
    }
}

pub struct Game {
    players_state: Vec<PlayerState>,
    ball: GameObject,
    top_bottom_bounds: [GameObject; 2],
    side_bounds: [GameObject; 2],
    camera: Camera2D,
    score: Score,
}

impl Game {
    pub fn new() -> Self {
        let ball = GameObject::from_pos(
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

        let score = Score::new();

        Self {
            players_state: vec![],
            ball,
            top_bottom_bounds,
            side_bounds,
            camera,
            score,
        }
    }

    pub fn add_player(&mut self, user_type: UserType, position: PlayerPosition) {
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

        let ai = match user_type {
            UserType::Client(_) => PREDICT,
            UserType::Ai(ai) => ai,
        };

        let players_state = PlayerState {
            player,
            user_type,
            ai,
        };

        self.players_state.push(players_state);
    }
}

impl Game {
    pub fn update(&mut self) {

        self.handle_player_state();

        let player_objects: Vec<&GameObject> =
            self.players_state.iter().map(|player_state| &player_state.player.object).collect();

        let bounce_objects: Vec<&GameObject> = self
            .top_bottom_bounds
            .iter()
            .chain(player_objects)
            .collect();

        self.ball.handle_bounces(bounce_objects);

        self.score.handle_goals(&mut self.ball, &self.side_bounds);
    }

    pub fn update_camera(&mut self) {
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

    fn handle_player_state(&mut self) {
        for player_state in self.players_state.iter_mut() {
            match player_state.user_type {
                UserType::Client(control_type) => {
                    match control_type {
                        ControlType::Mouse => player_state.player.mouse_control(),
                        ControlType::Keyboard(up, down) => player_state.player.keyboard_control(up, down),
                    }
                },
                UserType::Ai(_) => {
                    player_state.ai.behavior.observe(&player_state.player, &self.ball);
                    player_state.player.ai_control(&player_state.ai);
                },
            }
        }
    }

    fn show_bounds(&self) {
        for bound in self.top_bottom_bounds.iter() {
            bound.show_object(WHITE);
        }
    }

    fn show_players(&self) {
        for player_state in self.players_state.iter() {
            player_state.player.object.show_object(WHITE);
        }
    }

    fn show_ball(&self) {
        self.ball.show_object(WHITE);
    }

    fn show_score(&self) {
        self.score.show();
    }

    pub fn show(&self) {
        self.show_score();
        self.show_bounds();
        self.show_players();
        self.show_ball();
    }
}
