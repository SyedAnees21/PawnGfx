use std::ops::{Add, BitAnd, Mul, Shr, Sub};

#[derive(Clone, Copy)]
pub struct Color(f32, f32, f32, f32);

impl Default for Color {
	fn default() -> Self {
		Color::WHITE
	}
}

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

	#[inline(always)]
	pub fn add_raw(self, other: Color) -> Color {
		Color(
			self.0 + other.0,
			self.1 + other.1,
			self.2 + other.2,
			self.3 + other.3,
		)
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

#[derive(Default, Clone, Copy)]
pub struct Color32(pub u32);

impl BitAnd<u32> for Color32 {
	type Output = u32;

	#[inline(always)]
	fn bitand(self, rhs: u32) -> Self::Output {
		self.0 & rhs
	}
}

impl Shr<u32> for Color32 {
	type Output = u32;
	#[inline(always)]
	fn shr(self, rhs: u32) -> Self::Output {
		self.0 >> rhs
	}
}

impl Color32 {
	pub const C_MASK: u32 = 0x00FF00FF;
	const ROUND_MASK: u64 = 0x0080_0080;

	pub fn new(r: u32, g: u32, b: u32, a: u32) -> Color32 {
		Color32(r | (g << 8) | (b << 16) | (a << 24))
	}

	#[inline(always)]
	pub fn unpack(&self) -> Color {
		const INV_255: f32 = 1.0 / 255.0;
		Color(
			(*self & 0xFF) as f32 * INV_255,         // R
			((*self >> 8) & 0xFF) as f32 * INV_255,  // G
			((*self >> 16) & 0xFF) as f32 * INV_255, // B
			((*self >> 24) & 0xFF) as f32 * INV_255, // A
		)
	}

	/// SWAR based linear iterpolation for a 32 bit fixed point
	/// color.
	#[inline(always)]
	pub fn lerp(c0: Color32, c1: Color32, t: u32) -> Color32 {
		let rb_0 = c0 & 0x00FF00FF;
		let ag_0 = (c0 >> 8) & 0x00FF00FF;

		let rb_1 = c1 & 0x00FF00FF;
		let ag_1 = (c1 >> 8) & 0x00FF00FF;

		let rb = rb_0 + ((t * rb_1.wrapping_sub(rb_0)) >> 8);
		let ag = ag_0 + ((t * ag_1.wrapping_sub(ag_0)) >> 8);

		// (((c0 & 0x00FF00FF) * inv + (c1 & 0x00FF00FF) * t) >> 8) & 0x00FF00FF;
		// let ag = (((c0 >> 8) & 0x00FF00FF) * inv + ((c1 >> 8) & 0x00FF00FF) * t)
		// & 0xFF00FF00;

		Color32((rb & 0x00FF00FF) | ((ag & 0x00FF00FF) << 8))
		// Color32(rb | ag)
	}

	#[inline(always)]
	pub fn bi_lerp(
		c00: Color32,
		c01: Color32,
		c10: Color32,
		c11: Color32,
		tx: u32,
		ty: u32,
	) -> Color32 {
		let top = Color32::lerp(c00, c10, tx);
		let bottom = Color32::lerp(c01, c11, tx);

		Color32::lerp(top, bottom, ty)
	}

	#[inline(always)]
	pub fn pack(r: u32, g: u32, b: u32, a: u32) -> Color32 {
		Color32(r | (g << 8) | (b << 16) | (a << 24))
	}
}

impl Add for Color32 {
	type Output = Color32;

	#[inline(always)]
	fn add(self, rhs: Color32) -> Color32 {
		let a_1 = self.0 as u64;
		let a_2 = rhs.0 as u64;

		const R_MSK: u64 = 0xFF;
		const G_MSK: u64 = 0xFF00;
		const B_MSK: u64 = 0xFF0000;
		const A_MSK: u64 = 0xFF000000;

		let r = (((a_1 & R_MSK) + (a_2 & R_MSK)).min(255)) as u32;
		let g = (((a_1 & G_MSK) + (a_2 & G_MSK)).min(255)) as u32;
		let b = (((a_1 & B_MSK) + (a_2 & B_MSK)).min(255)) as u32;
		let a = (((a_1 & A_MSK) + (a_2 & A_MSK)).min(255)) as u32;

		// let r = ((a_1 & 0xFF) + (a_2 & 0xFF)).min(255);
		// let g = (((a_1 >> 8) & 0xFF) + ((a_2 >> 8) & 0xFF)).min(255);
		// let bch = (((a_1 >> 16) & 0xFF) + ((a_2 >> 16) & 0xFF)).min(255);
		// let a_ch = (((a_1 >> 24) & 0xFF) + ((a_2 >> 24) & 0xFF)).min(255);

		Color32(r | g | b | a)
		// Color32::pack(r, g, bch, a_ch)
	}
}

impl Sub for Color32 {
	type Output = Color32;

	#[inline(always)]
	fn sub(self, rhs: Color32) -> Color32 {
		let a_1 = self.0 as u64;
		let a_2 = rhs.0 as u64;

		const R_MSK: u64 = 0xFF;
		const G_MSK: u64 = 0xFF00;
		const B_MSK: u64 = 0xFF0000;
		const A_MSK: u64 = 0xFF000000;

		let r = (((a_1 & R_MSK) - (a_2 & R_MSK)).min(255)) as u32;
		let g = (((a_1 & G_MSK) - (a_2 & G_MSK)).min(255)) as u32;
		let b = (((a_1 & B_MSK) - (a_2 & B_MSK)).min(255)) as u32;
		let a = (((a_1 & A_MSK) - (a_2 & A_MSK)).min(255)) as u32;

		// let r = (a & 0xFF).saturating_sub(b & 0xFF);
		// let g = ((a >> 8) & 0xFF).saturating_sub((b >> 8) & 0xFF);
		// let bch = ((a >> 16) & 0xFF).saturating_sub((b >> 16) & 0xFF);
		// let a_ch = ((a >> 24) & 0xFF).saturating_sub((b >> 24) & 0xFF);

		Color32(r | g | b | a)
		// Color32::pack(r, g, bch, a_ch)
	}
}

impl Mul<f32> for Color32 {
	type Output = Color32;

	#[inline(always)]
	fn mul(self, scalar: f32) -> Color32 {
		let t = (scalar.clamp(0.0, 1.0) * 256.0 + 0.5) as u64;
		let a = self.0 as u64;

		let rb = (a & 0x00FF00FF) * t;
		let ag = ((a >> 8) & 0x00FF00FF) * t;

		let rb = (rb + Color32::ROUND_MASK) >> 8;
		let ag = (ag + Color32::ROUND_MASK) >> 8;

		Color32(((rb & 0x00FF00FF) | ((ag & 0x00FF00FF) << 8)) as u32)
	}
}

impl Mul for Color32 {
	type Output = Color32;

	#[inline(always)]
	fn mul(self, rhs: Color32) -> Color32 {
		let a = self.0 as u64;
		let b = rhs.0 as u64;

		let a_rb = a & 0x00FF00FF;
		let a_ag = (a >> 8) & 0x00FF00FF;
		let b_rb = b & 0x00FF00FF;
		let b_ag = (b >> 8) & 0x00FF00FF;

		let rb = a_rb * b_rb;
		let ag = a_ag * b_ag;

		let rb = rb + Color32::ROUND_MASK;
		let ag = ag + Color32::ROUND_MASK;

		let rb = (rb + (rb >> 8)) >> 8;
		let ag = (ag + (ag >> 8)) >> 8;

		Color32(((rb & 0x00FF00FF) | ((ag & 0x00FF00FF) << 8)) as u32)
	}
}
