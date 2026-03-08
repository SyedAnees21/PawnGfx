use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Color(f32, f32, f32, f32);

#[allow(unused)]
impl Color {
	pub const BLACK: Color = Color(0.0, 0.0, 0.0, 1.0);
	pub const BLUE: Color = Color(0.0, 0.0, 1.0, 1.0);
	pub const CYAN: Color = Color(0.0, 1.0, 1.0, 1.0);
	pub const GREEN: Color = Color(0.0, 1.0, 0.0, 1.0);
	pub const MAGENTA: Color = Color(1.0, 0.0, 1.0, 1.0);
	pub const RED: Color = Color(1.0, 0.0, 0.0, 1.0);
	pub const WHITE: Color = Color(1.0, 1.0, 1.0, 1.0);
	pub const YELLOW: Color = Color(1.0, 1.0, 0.0, 1.0);
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

	#[inline]
	pub fn new_rgb(r: f32, g: f32, b: f32) -> Self {
		Color(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0), 1.0)
	}

	#[inline]
	pub fn new_rgb_splat(v: f32) -> Self {
		Self::new_rgb(v, v, v)
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

	pub fn from_hex_unchecked(hex: &str) -> Color {
		Color::from_hex(hex).unwrap()
	}
}

impl Color {
	#[inline(always)]
	pub fn to_rgba8(&self) -> [u8; 4] {
		[
			(self.0.clamp(0.0, 1.0) * 255.0) as u8,
			(self.1.clamp(0.0, 1.0) * 255.0) as u8,
			(self.2.clamp(0.0, 1.0) * 255.0) as u8,
			(self.3.clamp(0.0, 1.0) * 255.0) as u8,
		]
	}
}

impl Add for Color {
	type Output = Color;

	#[inline(always)]
	fn add(self, other: Color) -> Color {
		Color(
			self.0 + other.0,
			self.1 + other.1,
			self.2 + other.2,
			self.3 + other.3,
		)
	}
}

impl Sub for Color {
	type Output = Color;

	#[inline(always)]
	fn sub(self, other: Color) -> Color {
		Color(
			self.0 - other.0,
			self.1 - other.1,
			self.2 - other.2,
			self.3 - other.3,
		)
	}
}

impl Mul<f32> for Color {
	type Output = Color;

	#[inline(always)]
	fn mul(self, scalar: f32) -> Color {
		Color(self.0 * scalar, self.1 * scalar, self.2 * scalar, self.3)
	}
}

impl Mul for Color {
	type Output = Color;

	#[inline(always)]
	fn mul(self, rhs: Self) -> Self::Output {
		Color(
			self.0 * rhs.0,
			self.1 * rhs.1,
			self.2 * rhs.2,
			self.3 * rhs.3,
		)
	}
}
