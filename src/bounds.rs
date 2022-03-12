use macroquad::prelude::{screen_height, screen_width, vec2, Vec2};

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Bounds {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }
}

impl Bounds {
    pub fn width(&self) -> f32 {
        self.w - self.x
    }

    pub fn height(&self) -> f32 {
        self.h - self.y
    }

    pub fn center(&self) -> Vec2 {
        let x = self.width() / 2.0;
        let y = self.height() / 2.0;
        vec2(x, y)
    }

    pub fn screen_ratio(&self) -> f32 {
        (screen_width() / self.w).min(screen_height() / self.h)
    }

    pub fn screen_size(&self) -> Vec2 {
        let ratio = self.screen_ratio();
        vec2(self.w * ratio, self.h * ratio)
    }

    pub fn screen_offset(&self) -> Vec2 {
        let xy = self.screen_size();
        (vec2(screen_width(), screen_height()) - xy) / 2.0
    }

    pub fn convert_to_local(&self, position: Vec2) -> Vec2 {
        (position - self.screen_offset()) / self.screen_ratio()
    }
}
