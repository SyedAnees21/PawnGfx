use {
	crate::math::Arithmetic,
	std::ops::{Add, Mul},
};

#[inline(always)]
pub fn lerp<T>(a: T, b: T, t: f32) -> T
where
	T: Copy + Arithmetic,
{
	a * (1.0 - t) + b * t
}

#[inline(always)]
pub fn bi_lerp<T>(c00: T, c01: T, c10: T, c11: T, dx: f32, dy: f32) -> T
where
	T: Copy + Arithmetic,
{
	let a = lerp(c00, c10, dx);
	let b = lerp(c01, c11, dx);

	lerp(a, b, dy)
}

#[inline(always)]
pub fn barycentric_interpolate<T>(
	w0: f32,
	w1: f32,
	w2: f32,
	v0: T,
	v1: T,
	v2: T,
) -> T
where
	T: Mul<f32, Output = T> + Add<Output = T>,
{
	v0 * w0 + v1 * w1 + v2 * w2
}

#[inline(always)]
pub fn perspective_interpolate<T>(
	bary: (f32, f32, f32),
	inv_d_lerped: f32,
	elements: (T, T, T),
) -> T
where
	T: Mul<f32, Output = T> + Add<Output = T> + Copy,
{
	let (w0, w1, w2) = bary;
	let (v0, v1, v2) = elements;
	let v_prime = barycentric_interpolate(w0, w1, w2, v0, v1, v2);

	v_prime * (1.0 / inv_d_lerped)
}

/// SWAR (SIMD Within A Register) lerp for packed 8-bit channels in a u32.
/// Expects `t` in the range 0..=255 (where 0 = a, 255 = b).
#[inline(always)]
pub fn lerp_swar_u32(a: u32, b: u32, t: u8) -> u32 {
	let t = t as u64;
	let inv = 256u64 - t;

	let a_lo = (a & 0x00FF00FF) as u64;
	let a_hi = ((a >> 8) & 0x00FF00FF) as u64;
	let b_lo = (b & 0x00FF00FF) as u64;
	let b_hi = ((b >> 8) & 0x00FF00FF) as u64;

	let lo = (a_lo * inv + b_lo * t + 0x0080_0080) >> 8;
	let hi = (a_hi * inv + b_hi * t + 0x0080_0080) >> 8;

	((lo as u32) & 0x00FF00FF) | (((hi as u32) & 0x00FF00FF) << 8)
}

/// Convenience wrapper for SWAR lerp using `t` in [0.0, 1.0].
#[inline(always)]
pub fn lerp_swar_u32_f32(a: u32, b: u32, t: f32) -> u32 {
	let t = (t.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
	lerp_swar_u32(a, b, t)
}
