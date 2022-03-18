use crate::{ai::Ai, bounds::Bounds};

pub static BALL_RADIUS: f32 = 8.0;
pub static BALL_SIZE: (f32, f32) = (BALL_RADIUS * 2.0, BALL_RADIUS * 2.0);

pub static PLAYER_PADDING: f32 = 50.0;
pub static PLAYER_WIDTH: f32 = 15.0;
pub static PLAYER_HEIGHT: f32 = 70.0;
pub static PLAYER_VELOCITY: (f32, f32) = (0.0, 2000.0);
pub static PLAYER_ACCELERATION: (f32, f32) = (0.0, 20000.0);

pub static BOUNDS: Bounds = Bounds::new(0.0, 0.0, 1200.0, 1000.0);
pub static BOUNDS_THICKNESS: f32 = 1000.0;

pub static MENU_SIZE: (f32, f32) = (300.0, 500.0);

pub const SARAH: Ai = Ai::new("Sarah", (0.1, 0.9), 0.95, 150);
pub const RAPHAEL: Ai = Ai::new("Raphael", (-0.1, 1.1), 0.5, 500);
pub const PREDICT: Ai = Ai::new("Predict", (0.5, 0.5), 1.0, 0);
