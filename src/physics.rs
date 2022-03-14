use macroquad::prelude::{draw_circle, draw_rectangle, vec2, Color, Rect, Vec2};

use crate::bounds::Bounds;

#[derive(Debug)]
pub enum ColliderType {
    Rectangle(f32, f32),
    Circle(f32),
}

#[derive(Debug)]
pub enum CollisionType {
    Vertical,
    Horizontal,
}

#[derive(Debug)]
pub struct Collider {
    pub shape: ColliderType,
    pub rect: Rect,
}

impl Collider {
    pub fn new(x: f32, y: f32, shape: ColliderType) -> Self {
        match shape {
            ColliderType::Rectangle(width, height) => Self {
                shape,
                rect: Rect::new(x, y, width, height),
            },
            ColliderType::Circle(radius) => Self {
                shape,
                rect: Rect::new(x, y, radius * 2.0, radius * 2.0),
            },
        }
    }
}

impl Collider {
    pub fn update_pos(&mut self, position: Vec2) {
        self.rect.move_to(position);
    }
}

#[derive(Debug)]
pub struct GameObject {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub collider: Collider,
    pub is_player: bool,
}

impl GameObject {
    pub fn from_pos(x: f32, y: f32, shape: ColliderType) -> Self {
        Self {
            position: vec2(x, y),
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(0.0, 0.0),
            collider: Collider::new(x, y, shape),
            is_player: false,
        }
    }
}

impl GameObject {
    pub fn show_object(&self, color: Color) {
        match self.collider.shape {
            ColliderType::Rectangle(w, h) => {
                draw_rectangle(self.position.x, self.position.y, w, h, color)
            }
            ColliderType::Circle(r) => {
                draw_circle(self.position.x + r, self.position.y + r, r, color);
            }
        }
    }

    pub fn move_object(&mut self, frame_time: f32) {
        self.position += self.velocity * frame_time;
        self.collider.update_pos(self.position);
    }

    pub fn check_collisions(&self, object: &Self) -> Option<CollisionType> {
        let collision_rect = self.collider.rect.intersect(object.collider.rect);

        if let Some(col) = collision_rect {
            if col.w < col.h {
                Some(CollisionType::Horizontal)
            } else {
                Some(CollisionType::Vertical)
            }
        } else {
            None
        }
    }

    pub fn check_collisions_vec(&self, objects: Vec<&Self>) -> Vec<CollisionType> {
        let mut collisions = Vec::new();
        for object in objects {
            let collision = self.check_collisions(object);
            if let Some(collision) = collision {
                collisions.push(collision);
            }
        }
        collisions
    }

    pub fn handle_bounces(&mut self, with: Vec<&GameObject>, frame_time: f32) {
        self.move_object(frame_time);
        for object in with {
            let collision = self.check_collisions(object);
            if let Some(CollisionType::Vertical) = collision {
                if self.collider.rect.top() > object.collider.rect.top() {
                    self.position.y = 2.0 * (object.collider.rect.y + object.collider.rect.h)
                        - self.collider.rect.y;
                } else {
                    self.position.y = 2.0 * (object.collider.rect.y - self.collider.rect.h)
                        - self.collider.rect.y;
                }

                self.velocity.y *= -1.0;
                self.velocity.y += object.velocity.y * 1.1;
            } else if let Some(CollisionType::Horizontal) = collision {
                if self.collider.rect.left() > object.collider.rect.left() {
                    self.position.x = 2.0 * (object.collider.rect.x + object.collider.rect.w)
                        - self.collider.rect.x;
                } else {
                    self.position.x = 2.0 * (object.collider.rect.x - self.collider.rect.w)
                        - self.collider.rect.x;
                }

                self.velocity.x *= -1.0;
                self.velocity.x += object.velocity.x * 1.1;

                if object.is_player {
                    self.velocity.y = ((self.collider.rect.y + self.collider.rect.h / 2.0)
                        - (object.collider.rect.y + object.collider.rect.h / 2.0))
                        * self.velocity.x.abs()
                        / 25.0;
                }
            }
        }
    }

    pub fn move_towards(
        &mut self,
        position: Vec2,
        velocity: Vec2,
        acceleration: Vec2,
        frame_time: f32,
    ) {
        self.acceleration = ((position - self.position) * 1000.0 - self.velocity * 88.0)
            .round()
            .clamp(-acceleration, acceleration);
        self.velocity += self.acceleration * frame_time;
        self.velocity = self.velocity.clamp(-velocity, velocity);
        self.move_object(frame_time);
    }

    pub fn move_towards_in_bounds(
        &mut self,
        position: Vec2,
        velocity: Vec2,
        acceleration: Vec2,
        bounds: Bounds,
        frame_time: f32,
    ) {
        let clamped_position = position.clamp(
            vec2(bounds.x, bounds.y),
            vec2(
                bounds.w - self.collider.rect.w,
                bounds.h - self.collider.rect.h,
            ),
        );
        self.move_towards(clamped_position, velocity, acceleration, frame_time)
    }
}
