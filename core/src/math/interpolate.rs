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
