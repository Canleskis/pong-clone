use std::f32::{INFINITY, consts::PI};

use ::rand::prelude::*;
use macroquad::{prelude::*, ui::{root_ui, widgets::{Button, Window, Group, Label}}, hash};

#[derive(Debug)]
enum ColliderType {
    Rectangle(f32, f32),
    Sphere(f32),
}

#[derive(Debug)]
enum CollisionType {
    Vertical(),
    Horizontal(),
}

#[derive(Debug)]
struct Collider {
    shape: ColliderType,
    rect: Rect,
}

impl Collider {
    fn new(x: f32, y: f32, shape: ColliderType) -> Self {
        match shape {
            ColliderType::Rectangle(width, height) => {
                Self {
                    shape,
                    rect: Rect::new(x, y, width, height),
                }
            },
            ColliderType::Sphere(radius) => {
                Self {
                    shape,
                    rect: Rect::new(x, y, radius, radius)
                }
            },
        }
    }
}

impl Collider {
    fn update_pos(&mut self, position: Vec2) {
        match self.shape {
            ColliderType::Rectangle(width, height) => {
                self.rect = Rect::new(position.x, position.y, width, height);
            }
            ColliderType::Sphere(radius) => {
                self.rect = Rect::new(position.x - radius, position.y - radius, radius * 2.0, radius * 2.0);
            },
        }
    }
}

#[derive(Debug)]
struct GameObject {
    position: Vec2,
    velocity: Vec2,
    collider: Collider,
    is_player: bool,
}

impl GameObject {
    fn from_pos(x: f32, y: f32, shape: ColliderType) -> Self {
        Self {
            position: vec2(x, y),
            velocity: vec2(0.0, 0.0),
            collider: Collider::new(x, y, shape),
            is_player: false,
        }
    }
}

impl GameObject {
    fn show_object(&self) {
        match self.collider.shape {
            ColliderType::Rectangle(w, h) => {
                draw_rectangle(self.position.x, self.position.y, w, h, WHITE)
            },
            ColliderType::Sphere(r) => {
                draw_circle(self.position.x, self.position.y, r, WHITE);
            },
        }
    }

    fn move_object(&mut self, frame_time: f32) {
        self.position += self.velocity * frame_time;
        self.collider.update_pos(self.position);
    }

    fn check_collisions(&self, object: &Self) -> Option<CollisionType> {
        let collision_rect = self.collider.rect.intersect(object.collider.rect);

        if let Some(col) = collision_rect {
            if col.w < col.h {
                Some(CollisionType::Horizontal())
            } else {
                Some(CollisionType::Vertical())
            }
        } else {
            None
        }
    }

    fn collision_rect(&self, object: &Self) -> Option<Rect> {
        self.collider.rect.intersect(object.collider.rect)
    }

    fn check_collisions_vec(&self, objects: Vec<&Self>) -> Vec<CollisionType> {
        let mut collisions = Vec::new();
        for object in objects {
            let collision = self.check_collisions(object);
            if let Some(collision) = collision {
                collisions.push(collision);
            }
        }
        collisions
    }
    // Put this in ball struct
    fn handle_bounces(&mut self, with: Vec<&GameObject>, frame_time: f32) {
        for object in with {
            let collision = self.check_collisions(object);
            if let Some(CollisionType::Vertical()) = collision {
                if self.collider.rect.top() > object.collider.rect.top() {
                    self.position.y = object.collider.rect.bottom() + self.collider.rect.h / 2.0;

                } else if self.collider.rect.top() < object.collider.rect.top() {
                    self.position.y = object.collider.rect.top() - self.collider.rect.h / 2.0;
                }
                self.velocity.y *= -1.0;
                self.velocity.y += object.velocity.y * 1.1;

            } else if let Some(CollisionType::Horizontal()) = collision {
                if self.collider.rect.right() > object.collider.rect.right() {
                    self.position.x = object.collider.rect.right() + self.collider.rect.w / 2.0;

                } else if self.collider.rect.right() < object.collider.rect.right() {
                    self.position.x = object.collider.rect.left() - self.collider.rect.w / 2.0;
                }
                // TODO: Add left comparison to handle 0 width objects and replace / 2.0 with actual width (rework probable)
                self.velocity.x *= -1.0;
                self.velocity.x += object.velocity.x * 1.1;
                if object.is_player {
                    self.velocity.y = (self.position.y - (object.position.y + object.collider.rect.h / 2.0)) * self.velocity.x.abs() / 30.0;
                }
            }
            
        }
        self.move_object(frame_time)
    }
}

struct Player {
    object: GameObject,
    score: u8,
    bounds: Bounds,
}

impl Player {
    fn new(mut object: GameObject, bounds: Bounds) -> Self {
        object.is_player = true;
        Self {
            object,
            score: 0,
            bounds,
        }
    }
}

impl Player {
    fn scored(&mut self) {
        self.score += 1;
    }

    fn move_towards(&mut self, position: Vec2, velocity: Vec2, smoothing: u8, frame_time: f32) {
        let smoothing_factor = 100.0 / ((smoothing + 1) as f32);
        self.object.velocity = (smoothing_factor * (position - self.object.position)).clamp(-velocity, velocity);
        self.object.move_object(frame_time)
    }

    fn move_towards_in_bounds(&mut self, position: Vec2, velocity: Vec2, smoothing: u8, frame_time: f32) {
        let clamped_position = position.clamp(vec2(self.bounds.x1, self.bounds.y1), vec2(self.bounds.x2, self.bounds.y2 - self.object.collider.rect.h));
        self.move_towards(clamped_position, velocity, smoothing, frame_time)
    }
}

#[derive(Debug, Clone, Copy)]
struct Bounds {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

impl Bounds {
    fn width(&self) -> f32 {
        self.x2 - self.x1
    }

    fn height(&self) -> f32 {
        self.y2 - self.y1
    }

    fn center(&self) -> Vec2 {
        let x = self.width() / 2.0;
        let y = self.height() / 2.0;
        vec2(x, y)
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    println!("_________New game_________");

    let bounds = Bounds {
        x1: 0.0,
        y1: 0.0,
        x2: 1280.0,
        y2: 720.0,
    };

    let bounds_thickness = 100.0;
    
    let player_width = 15.0;
    let player_height = 100.0;
    let player_padding = 50.0;
    let player_velocity = vec2(0.0, 1200.0);
    let ball_size = 8.0;

    let top_bound = GameObject::from_pos(bounds.x1, bounds.y1 - bounds_thickness, ColliderType::Rectangle(bounds.x2, bounds_thickness));
    let bottom_bound = GameObject::from_pos(bounds.x1, bounds.y2, ColliderType::Rectangle(bounds.x2, bounds_thickness));

    let left_bound = GameObject::from_pos(bounds.x1 - bounds_thickness, bounds.y1, ColliderType::Rectangle(bounds_thickness, bounds.y2));
    let right_bound = GameObject::from_pos(bounds.x2, bounds.y1, ColliderType::Rectangle(bounds_thickness, bounds.y2));

    let paddle_left = GameObject::from_pos(bounds.x1 + player_padding, bounds.center().y - player_height / 2.0, ColliderType::Rectangle(player_width, player_height));
    let paddle_right = GameObject::from_pos(bounds.x2 - player_padding - player_width, bounds.center().y - player_height / 2.0, ColliderType::Rectangle(player_width, player_height));

    let mut player_left = Player::new(paddle_left, bounds);
    let mut player_right = Player::new(paddle_right, bounds);

    let mut ball = GameObject::from_pos(bounds.center().x, bounds.center().y, ColliderType::Sphere(ball_size));

    let mut score_time = get_time();
    let mut random_start = match thread_rng().gen_range(1..=2) {
        1 => -1,
        _ => 1
    };

    let mut game_paused = false;

    let mut ai1_move = true;
    let mut ai2_move = true;

    let mut hit_position1 = 0.5;
    let mut hit_position2 = 0.5;

    let mut rest_position1 = 0.5;
    let mut rest_position2 = 0.5;

    let ai_smoothing = 0;

    let mut max_velocity = 0.0;

    loop {
        let frame_time = get_frame_time();

        if is_key_pressed(KeyCode::Escape) {
            if game_paused {
                game_paused = false;
            } else {
                game_paused = true
            }
        }

        if !game_paused {

            let reset_button = root_ui().button(vec2(0.0, 0.0), "Reset ball");

            // Group::new(hash!("shop", 0), Vec2::new(300., 80.)).ui(&mut *root_ui(), |ui| {
            //     Label::new("Score").position(bounds.center());
            // });

            let mut player_controlled = true;
            
            if is_mouse_button_down(MouseButton::Left) {
                player_left.move_towards_in_bounds(mouse_position().into(), player_velocity, 5, frame_time);
            } else if is_key_down(KeyCode::W) {
                player_left.move_towards_in_bounds(vec2(player_left.object.position.y, -INFINITY), player_velocity, 5, frame_time);
            } else if is_key_down(KeyCode::S) {
                player_left.move_towards_in_bounds(vec2(player_left.object.position.y, INFINITY), player_velocity, 5, frame_time);
            } else if is_key_down(KeyCode::D) {
                player_left.move_towards_in_bounds(vec2(bounds.x2, player_left.object.position.y), player_velocity, 5, frame_time);
            } else {
                player_controlled = false;
                player_left.object.velocity = vec2(0.0, 0.0);
            }

            if is_mouse_button_down(MouseButton::Right) {
                player_right.move_towards_in_bounds(mouse_position().into(), player_velocity, 5, frame_time);
            } else if is_key_down(KeyCode::Up) {
                player_right.move_towards_in_bounds(vec2(player_right.object.position.y, -INFINITY), player_velocity, 5, frame_time);
            } else if is_key_down(KeyCode::Down) {
                player_right.move_towards_in_bounds(vec2(player_right.object.position.y, INFINITY), player_velocity, 5, frame_time);
            } else {
                player_right.object.velocity = vec2(0.0, 0.0);
            }

            ball.handle_bounces(vec![&player_left.object, &player_right.object, &top_bound, &bottom_bound], frame_time);

            if !player_controlled {
                if ai1_move {
                    player_left.move_towards_in_bounds(vec2(player_left.object.position.x, ball.position.y - player_height * hit_position1), player_velocity, ai_smoothing, frame_time);
                } else {
                    player_left.move_towards_in_bounds(vec2(player_left.object.position.x, (bounds.y2 - player_height) * rest_position1), player_velocity * 0.5, ai_smoothing, frame_time);
                }
            }

            if ai2_move {
                player_right.move_towards_in_bounds(vec2(player_right.object.position.x, ball.position.y - player_height * hit_position2), player_velocity, ai_smoothing, frame_time);
            } else {
                player_right.move_towards_in_bounds(vec2(player_right.object.position.x, (bounds.y2 - player_height) * rest_position2), player_velocity * 0.5, ai_smoothing, frame_time);
            }

            if ball.check_collisions(&player_left.object).is_some() {
                hit_position2 = thread_rng().gen_range(0.0..=1.0);
                rest_position2 = thread_rng().gen_range(0.0..=1.0);
                ai1_move = false;
                ai2_move = true;
            }

            if ball.check_collisions(&player_right.object).is_some() {
                hit_position1 = thread_rng().gen_range(0.0..=1.0);
                rest_position1 = thread_rng().gen_range(0.0..=1.0);
                ai1_move = true;
                ai2_move = false;
            }

            if ball.check_collisions(&left_bound).is_some() {
                player_right.scored();
                score_time = get_time();
            }

            if ball.check_collisions(&right_bound).is_some() {
                player_left.scored();
                score_time = get_time();
            }

            if reset_button {
                score_time = get_time();
            }

            if ball.velocity.y > max_velocity {
                max_velocity = ball.velocity.y;
                println!("{}", max_velocity);
            }
            
            if score_time != 0.0 {
                ball.position = bounds.center();
                ball.velocity = vec2(0.0, 0.0);

                if get_time() > score_time + 1.0 {
                    let ball_velocity = vec2((random_start as f32) * 1000.0, thread_rng().gen_range(-600.0..=600.0));
                    ball.velocity = ball_velocity;
                    random_start *= -1;
                    score_time = 0.0;
                }
                ai1_move = true;
                ai2_move = true;
            }
        } else {
            let resume_button = root_ui().button(bounds.center(), "Resume");
            if resume_button {
                game_paused = false;
            }
        }

        draw_text(&player_left.score.to_string(), (bounds.center().x + bounds.x1) / 2.0, bounds.center().y / 2.0, 60.0, WHITE);
        draw_text(&player_right.score.to_string(), (bounds.x2 + bounds.center().x) / 2.0, bounds.center().y / 2.0, 60.0, WHITE);

        player_left.object.show_object();
        player_right.object.show_object();
        ball.show_object();

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong Clone".to_string(),
        window_width: 1280,
        window_height: 720,
        //window_resizable: false,
        ..Default::default()
    }
}