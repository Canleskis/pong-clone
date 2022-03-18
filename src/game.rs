use macroquad::{
    camera::{set_camera, Camera2D},
    prelude::{get_frame_time, Rect, WHITE, scene::{Node, RefMut}},
};

use crate::{
    constants::{
        BALL_RADIUS, BOUNDS, BOUNDS_THICKNESS, PLAYER_ACCELERATION, PLAYER_HEIGHT, PLAYER_PADDING,
        PLAYER_VELOCITY, PLAYER_WIDTH, PREDICT,
    },
    physics::{ColliderType, GameObject},
    player::{ControlType, Player, PlayerPosition, PlayerState, UserType},
    score::Score,
};

pub struct Game {
    players_state: Vec<PlayerState>,
    player_amount: u8,
    ball: GameObject,
    top_bottom_bounds: [GameObject; 2],
    side_bounds: [GameObject; 2],
    camera: Camera2D,
    score: Score,
    game_time: f32,
}

impl Game {
    pub fn new() -> Self {
        let ball = GameObject::from_pos(
            BOUNDS.center().0 - BALL_RADIUS,
            BOUNDS.center().1 - BALL_RADIUS,
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
            player_amount: 0,
            ball,
            top_bottom_bounds,
            side_bounds,
            camera,
            score,
            game_time: 0.0,
        }
    }

    pub fn add_player(&mut self, user_type: UserType, position: PlayerPosition) -> u8 {
        let player_position = match position {
            PlayerPosition::Left => BOUNDS.x + PLAYER_PADDING,
            PlayerPosition::Right => BOUNDS.w - PLAYER_PADDING - PLAYER_WIDTH,
        };

        let (ai, name) = match user_type {
            UserType::Client(_) => (PREDICT, "User1"),
            UserType::Ai(ai) => (ai, ai.name),
        };

        let player = Player::new(
            name,
            GameObject::from_pos(
                player_position,
                BOUNDS.center().1 - PLAYER_HEIGHT / 2.0,
                ColliderType::Rectangle(PLAYER_WIDTH, PLAYER_HEIGHT),
            ),
            BOUNDS,
            PLAYER_VELOCITY.into(),
            PLAYER_ACCELERATION.into(),
            position,
        );

        let id = self.player_amount;

        let player_state = PlayerState {
            user_type,
            player,
            ai,
            id,
        };
        self.players_state.push(player_state);
        self.player_amount += 1;

        id
    }

    pub fn remove_player(&mut self, id: u8) {
        self.players_state
            .retain(|player_sate| player_sate.id != id);
    }

    pub fn player_type(&self, id: u8) -> Option<UserType> {
        let matching_player_state = self
            .players_state
            .iter()
            .find(|player_state| player_state.id == id);
        if let Some(player_state) = matching_player_state {
            Some(player_state.user_type)
        } else {
            None
        }
    }
}

impl Game {
    pub fn update(&mut self) {
        self.game_time += get_frame_time();
        self.handle_player_state();
        self.score
            .handle_goals(&mut self.ball, &self.side_bounds, self.game_time);

        let player_objects: Vec<&GameObject> = self
            .players_state
            .iter()
            .map(|player_state| &player_state.player.object)
            .collect();

        let bounce_objects: Vec<&GameObject> = self
            .top_bottom_bounds
            .iter()
            .chain(player_objects)
            .collect();

        self.ball.handle_bounces(bounce_objects);
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
        let ball_collisions = self.ball.check_collisions_vec(
            self.players_state
                .iter()
                .map(|player_state| &player_state.player.object)
                .collect(),
        );

        for player_state in self.players_state.iter_mut() {
            match player_state.user_type {
                UserType::Client(control_type) => match control_type {
                    ControlType::Mouse => player_state.player.mouse_control(),
                    ControlType::Keyboard(up, down) => {
                        player_state.player.keyboard_control(up, down)
                    }
                },
                UserType::Ai(_) => {
                    player_state.ai.behavior.observe(
                        &player_state.player,
                        &self.ball,
                        &ball_collisions,
                        self.game_time,
                    );
                    player_state
                        .player
                        .ai_control(&player_state.ai, self.game_time);
                }
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

impl Node for Game {
    fn draw(mut node: RefMut<Self>) {
        node.update_camera();
        node.show();
    }

    fn update(mut node: RefMut<Self>) {
        node.update();
    }
}
