use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Color(f32, f32, f32, f32);

#[allow(unused)]
impl Color {
    pub const RED: Color = Color(1.0, 0.0, 0.0, 1.0);
    pub const BLUE: Color = Color(0.0, 0.0, 1.0, 1.0);
    pub const GREEN: Color = Color(0.0, 1.0, 0.0, 1.0);
    pub const WHITE: Color = Color(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Color = Color(0.0, 0.0, 0.0, 1.0);
    pub const YELLOW: Color = Color(1.0, 1.0, 0.0, 1.0);
    pub const CYAN: Color = Color(0.0, 1.0, 1.0, 1.0);
    pub const MAGENTA: Color = Color(1.0, 0.0, 1.0, 1.0);
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color(
            r.clamp(0.0, 1.0),
            g.clamp(0.0, 1.0),
            b.clamp(0.0, 1.0),
            a.clamp(0.0, 1.0),
        )
    }

    pub fn new_rgb(r: f32, g: f32, b: f32) -> Self {
        Color(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0), 1.0)
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        let len = hex.len();

        match len {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()? as f32 / 255.0;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()? as f32 / 255.0;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()? as f32 / 255.0;
                Some(Color::new_rgb(r, g, b))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()? as f32 / 255.0;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()? as f32 / 255.0;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()? as f32 / 255.0;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()? as f32 / 255.0;
                Some(Color::new(r, g, b, a))
            }
            _ => None,
        }
    }
}

impl Color {
    pub fn to_rgba8(&self) -> [u8; 4] {
        [
            (self.0 * 255.0) as u8,
            (self.1 * 255.0) as u8,
            (self.2 * 255.0) as u8,
            (self.3 * 255.0) as u8,
        ]
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color(
            (self.0 + other.0).min(1.0),
            (self.1 + other.1).min(1.0),
            (self.2 + other.2).min(1.0),
            (self.3 + other.3).min(1.0),
        )
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color(
            (self.0 - other.0).max(0.0),
            (self.1 - other.1).max(0.0),
            (self.2 - other.2).max(0.0),
            (self.3 - other.3).max(0.0),
        )
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, scalar: f64) -> Color {
        Color(
            self.0 * scalar as f32,
            self.1 * scalar as f32,
            self.2 * scalar as f32,
            self.3,
        )
    }
}

impl Mul for Color {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self
    }
}
