use crate::geometry::Normal;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Default)]
pub struct Normal32(pub u32);

impl Normal32 {
	pub fn new(x: f32, y: f32) -> Self {
		let ux = (((x * 0.5 + 0.5) * 65535.0) as u32).clamp(0, 65535);
		let uy = (((y * 0.5 + 0.5) * 65535.0) as u32).clamp(0, 65535);
		Self(ux | (uy << 16))
	}

	/// Packs a unit vector into 16-bit X and 16-bit Y.
	/// Range: [-1.0, 1.0] -> [0, 65535]
	pub fn pack(n: Normal) -> Self {
		let ux = (((n.x * 0.5 + 0.5) * 65535.0) as u32).clamp(0, 65535);
		let uy = (((n.y * 0.5 + 0.5) * 65535.0) as u32).clamp(0, 65535);
		Self(ux | (uy << 16))
	}

	/// Convert back to Vector3 and reconstruct Z
	#[inline(always)]
	pub fn unpack(&self) -> Normal {
		let nx = (self.0 & 0xFFFF) as f32 / 65535.0 * 2.0 - 1.0;
		let ny = (self.0 >> 16) as f32 / 65535.0 * 2.0 - 1.0;

		// Reconstruct Z = sqrt(1 - x^2 - y^2)
		// We clamp to 0.0 to avoid NaN if floating point drift makes it > 1.0
		let nz = (1.0 - nx * nx - ny * ny).max(0.0).sqrt();

		Normal::new(nx, ny, nz)
	}

	#[inline(always)]
	pub fn lerp(n0: Self, n1: Self, weight: u32) -> Self {
		let x0 = n0.0 & 0xFFFF;
		let y0 = n0.0 >> 16;
		let x1 = n1.0 & 0xFFFF;
		let y1 = n1.0 >> 16;

		// x = x0 + (weight * (x1 - x0) >> 8)
		let rx = x0.wrapping_add((weight.wrapping_mul(x1.wrapping_sub(x0))) >> 8);
		let ry = y0.wrapping_add((weight.wrapping_mul(y1.wrapping_sub(y0))) >> 8);

		Self((rx & 0xFFFF) | (ry << 16))
	}

	#[inline(always)]
	pub fn bi_lerp(
		n00: Normal32,
		n01: Normal32,
		n10: Normal32,
		n11: Normal32,
		tx: u32,
		ty: u32,
	) -> Normal32 {
		let top = Normal32::lerp(n00, n10, tx);
		let bottom = Normal32::lerp(n01, n11, tx);

		Normal32::lerp(top, bottom, ty)
	}
}

impl Add for Normal32 {
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: Self) -> Self {
		let x0 = (self.0 & 0xFFFF) as i32;
		let y0 = (self.0 >> 16) as i32;
		let x1 = (rhs.0 & 0xFFFF) as i32;
		let y1 = (rhs.0 >> 16) as i32;

		// Subtract bias (32767), add, then re-add bias
		// Clamping prevents values from bleeding into the other component's bits
		let rx = (x0 + x1 - 32767).clamp(0, 65535) as u32;
		let ry = (y0 + y1 - 32767).clamp(0, 65535) as u32;

		Self(rx | (ry << 16))
	}
}

impl Mul<f32> for Normal32 {
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: f32) -> Self {
		// Convert to signed space (-32767 to 32767), scale, then re-bias
		let x = (self.0 & 0xFFFF) as f32 - 32767.0;
		let y = (self.0 >> 16) as f32 - 32767.0;

		let rx = (x * rhs + 32767.0).clamp(0.0, 65535.0) as u32;
		let ry = (y * rhs + 32767.0).clamp(0.0, 65535.0) as u32;

		Self(rx | (ry << 16))
	}
}

impl Mul for Normal32 {
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: Self) -> Self {
		// Multiplicative blending in fixed point
		let x0 = ((self.0 & 0xFFFF) as f32 / 32767.0) - 1.0;
		let y0 = ((self.0 >> 16) as f32 / 32767.0) - 1.0;
		let x1 = ((rhs.0 & 0xFFFF) as f32 / 32767.0) - 1.0;
		let y1 = ((rhs.0 >> 16) as f32 / 32767.0) - 1.0;

		let rx = (((x0 * x1) + 1.0) * 32767.0).clamp(0.0, 65535.0) as u32;
		let ry = (((y0 * y1) + 1.0) * 32767.0).clamp(0.0, 65535.0) as u32;

		Self(rx | (ry << 16))
	}
}

impl Sub for Normal32 {
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self {
		// Extract components as signed integers to allow negative intermediate results
		let x0 = (self.0 & 0xFFFF) as i32;
		let y0 = (self.0 >> 16) as i32;
		let x1 = (rhs.0 & 0xFFFF) as i32;
		let y1 = (rhs.0 >> 16) as i32;

		// Logic: (val0 - bias) - (val1 - bias) + bias
		// Which simplifies to: val0 - val1 + 32767
		let rx = (x0 - x1 + 32767).clamp(0, 65535) as u32;
		let ry = (y0 - y1 + 32767).clamp(0, 65535) as u32;

		Self(rx | (ry << 16))
	}
}
