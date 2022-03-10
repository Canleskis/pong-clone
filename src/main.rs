use std::f32::{INFINITY};

use macroquad::{prelude::*, ui::{root_ui}, rand::gen_range};

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
            ColliderType::Rectangle(_, _) => {
                self.rect.move_to(position);
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
    fn show_object(&self, color: Color) {
        match self.collider.shape {
            ColliderType::Rectangle(w, h) => {
                draw_rectangle(self.position.x, self.position.y, w, h, color)
            },
            ColliderType::Sphere(r) => {
                draw_circle(self.position.x, self.position.y, r, color);
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
        self.move_object(frame_time);
        for object in with {
            let collision = self.check_collisions(object);
            if let Some(CollisionType::Vertical()) = collision {
                if self.collider.rect.top() > object.collider.rect.top() {
                    self.position.y = (2.0 * (object.collider.rect.y + object.collider.rect.h)) - self.collider.rect.y;
                } else {
                    self.position.y = (2.0 * (object.collider.rect.y - self.collider.rect.h)) - self.collider.rect.y;
                }
                self.velocity.y *= -1.0;
                self.velocity.y += object.velocity.y * 1.1;

            } else if let Some(CollisionType::Horizontal()) = collision {
                if self.collider.rect.left() > object.collider.rect.left() {
                    self.position.x = (2.0 * (object.collider.rect.x + object.collider.rect.w)) - self.collider.rect.x;
                } else {
                    self.position.x = (2.0 * (object.collider.rect.x - self.collider.rect.w)) - self.collider.rect.x;
                }
                self.velocity.x *= -1.0;
                self.velocity.x += object.velocity.x * 1.1;

                if object.is_player {
                    self.velocity.y = (self.position.y - (object.position.y + object.collider.rect.h / 2.0)) * self.velocity.x.abs() / 30.0;
                }
            }
        }
    }

    fn predict(&self, x: f32, bounds: Bounds) -> f32 {
        let slope = self.velocity.y / self.velocity.x;
        let f = -self.position.x * slope + self.position.y;
        let height = bounds.y2 - self.collider.rect.h;
        let y = ((slope * x + f) % (2.0 * height) + 2.0 * height) % (2.0 * height);
        y.min(2.0 * height - y)
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
        x2: 1200.0,
        y2: 1000.0,
    };

    let bounds_thickness = 100.0;
    
    let player_width = 15.0;
    let player_height = 80.0;
    let player_padding = 50.0;
    let player_velocity = vec2(0.0, 2000.0);
    let ball_size = 16.0;

    let top_bound = GameObject::from_pos(bounds.x1, bounds.y1 - bounds_thickness, ColliderType::Rectangle(bounds.x2, bounds_thickness));
    let bottom_bound = GameObject::from_pos(bounds.x1, bounds.y2, ColliderType::Rectangle(bounds.x2, bounds_thickness));

    let left_bound = GameObject::from_pos(bounds.x1 - bounds_thickness, bounds.y1 - 1E6, ColliderType::Rectangle(bounds_thickness, bounds.y2 + 2E6));
    let right_bound = GameObject::from_pos(bounds.x2, bounds.y1 - 1E6, ColliderType::Rectangle(bounds_thickness, bounds.y2 + 2E6));

    let paddle_left = GameObject::from_pos(bounds.x1 + player_padding, bounds.center().y - player_height / 2.0, ColliderType::Rectangle(player_width, player_height));
    let paddle_right = GameObject::from_pos(bounds.x2 - player_padding - player_width, bounds.center().y - player_height / 2.0, ColliderType::Rectangle(player_width, player_height));

    let mut player_left = Player::new(paddle_left, bounds);
    let mut player_right = Player::new(paddle_right, bounds);

    let mut ball = GameObject::from_pos(bounds.center().x, bounds.center().y, ColliderType::Rectangle(ball_size, ball_size));

    let mut score_time = get_time();
    let mut random_start = match gen_range(1, 2) {
        1 => -1,
        _ => 1
    };

    let mut game_paused = false;
    let mut show_prediction = false;
    let mut player_controlled = false;

    let ai_smoothing = 15;
    
    let mut hit_position1 = 0.5;
    let mut hit_position2 = 0.5;
    
    let mut camera = Camera2D::from_display_rect(Rect::new(bounds.x1, bounds.y1 - 1.0, bounds.x2, bounds.y2 + 2.0));

    loop {
        let ratio = (screen_width() / bounds.x2).min(screen_height() / bounds.y2);
        let x = bounds.x2 * ratio;
        let y = bounds.y2 * ratio;
        let a = (screen_width() - x) / 2.0;
        let b = (screen_height() - y) / 2.0;
        camera.viewport = Some((a as i32, b as i32, x as i32, y as i32));
        
        set_camera(&camera);

        let frame_time = get_frame_time();

        if is_key_pressed(KeyCode::Escape) {
            game_paused ^= true;
        }

        if !game_paused {

            let reset_button = root_ui().button(vec2(0.0, 0.0), "Reset ball");
            let show_prediction_button = root_ui().button(vec2(0.0, 20.0), "Show ball prediction");

            if show_prediction_button {
                show_prediction ^= true;
            }
            
            //___PHYSICS___//

            ball.handle_bounces(vec![&player_left.object, &player_right.object, &top_bound, &bottom_bound], frame_time);

            //___PLAYER INPUTS___//


            if is_mouse_button_pressed(MouseButton::Left) {
                player_controlled ^= true;
                if player_controlled {
                    score_time = get_time();
                }
            }
            
            if player_controlled {
                set_cursor_grab(true);
                show_mouse(false);
                let mut mouse_pos = (Vec2::from(mouse_position()) - vec2(a, b)) / ratio;
                if let Some(touch) = touches().first() {
                    mouse_pos = (Vec2::from(touch.position) - vec2(a, b)) / ratio;
                }

                player_left.move_towards_in_bounds(mouse_pos, player_velocity, 0, frame_time);
            } else if is_key_down(KeyCode::W) {
                player_left.move_towards_in_bounds(vec2(player_left.object.position.y, -INFINITY), player_velocity * 0.5, 5, frame_time);
            } else if is_key_down(KeyCode::S) {
                player_left.move_towards_in_bounds(vec2(player_left.object.position.y, INFINITY), player_velocity * 0.5, 5, frame_time);
            } else {
                set_cursor_grab(false);
                show_mouse(true);
                player_left.object.velocity = vec2(0.0, 0.0);
            }

            if is_mouse_button_down(MouseButton::Right) {
                player_right.move_towards_in_bounds(mouse_position().into(), player_velocity, 0, frame_time);
            } else if is_key_down(KeyCode::Up) {
                player_right.move_towards_in_bounds(vec2(player_right.object.position.y, -INFINITY), player_velocity, 5, frame_time);
            } else if is_key_down(KeyCode::Down) {
                player_right.move_towards_in_bounds(vec2(player_right.object.position.y, INFINITY), player_velocity, 5, frame_time);
            } else {
                player_right.object.velocity = vec2(0.0, 0.0);
            }

            //___AIs___//

            let prediction_difficulty = if ball.velocity != vec2(0.0, 0.0) {
                (ball.velocity.y / ball.velocity.x).abs() / 4.0
            } else {
                0.0
            };

            if !ball.check_collisions_vec(vec![&player_right.object, &top_bound, &bottom_bound]).is_empty() || score_time != 0.0 {
                let prediction_difficulty = -0.05;
                hit_position1 = gen_range(0.0 - prediction_difficulty, 1.0 + prediction_difficulty);
            }

            if !ball.check_collisions_vec(vec![&player_left.object, &top_bound, &bottom_bound]).is_empty() || score_time != 0.0 {
                hit_position2 = gen_range(0.0 - prediction_difficulty, 1.0 + prediction_difficulty);
            }
            
            let player_left_bounce_position = player_left.object.position.x + player_width;
            let ball_predicted_left = ball.predict(player_left_bounce_position, bounds);
            if !player_controlled {
                if ball.velocity.x < 0.0 {
                    player_left.move_towards_in_bounds(vec2(player_left.object.position.x, ball_size / 2.0 + ball_predicted_left - player_height * hit_position1), player_velocity, ai_smoothing, frame_time);
                } else {
                    player_left.move_towards_in_bounds(vec2(player_left.object.position.x, bounds.center().y - player_height / 2.0), player_velocity, ai_smoothing, frame_time);
                }
            }

            let player_right_bounce_position = player_right.object.position.x - player_width;
            let ball_predicted_right = ball.predict(player_right_bounce_position, bounds);
            if ball.velocity.x > 0.0 {
                player_right.move_towards_in_bounds(vec2(player_right.object.position.x, ball_size / 2.0 + ball_predicted_right - player_height * hit_position2), player_velocity, ai_smoothing, frame_time);
            } else {
                player_right.move_towards_in_bounds(vec2(player_right.object.position.x, bounds.center().y - player_height / 2.0), player_velocity, ai_smoothing, frame_time);
            }

            if show_prediction {                
                GameObject::from_pos(player_right_bounce_position, ball_predicted_right, ColliderType::Rectangle(ball_size, ball_size)).show_object(WHITE);
                GameObject::from_pos(player_left_bounce_position, ball_predicted_left, ColliderType::Rectangle(ball_size, ball_size)).show_object(WHITE);
            }

            //___SCORING___//

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
            
            if score_time != 0.0 {
                ball.position = bounds.center();
                ball.velocity = vec2(0.0, 0.0);

                if get_time() > score_time + 1.0 {
                    let ball_velocity = vec2((random_start as f32) * 1000.0, gen_range(-400.0, 400.0));
                    ball.velocity = ball_velocity;
                    random_start *= -1;
                    score_time = 0.0;
                }
            }
            
        } else {
            set_cursor_grab(false);
            show_mouse(true);
            let resume_button = root_ui().button(vec2(screen_width() / 2.0, screen_height() / 2.0), "Resume");
            if resume_button {
                game_paused = false;
            }
        }

        draw_text(&player_left.score.to_string(), (bounds.center().x + bounds.x1) / 2.0, bounds.center().y / 2.0, 60.0, WHITE);
        draw_text(&player_right.score.to_string(), (bounds.x2 + bounds.center().x) / 2.0, bounds.center().y / 2.0, 60.0, WHITE);

        player_left.object.show_object(WHITE);
        player_right.object.show_object(WHITE);
        ball.show_object(WHITE);
        top_bound.show_object(WHITE);
        bottom_bound.show_object(WHITE);

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong Clone".to_owned(),
        window_width: 1200,
        window_height: 1000,
        ..Default::default()
    }
}