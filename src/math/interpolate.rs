use std::ops::{Add, Mul, Sub};

use crate::math::Arithmetic;

#[inline(always)]
pub fn lerp<T>(a: T, b: T, t: f64) -> T
where
    T: Copy + Arithmetic,
{
    a * (1.0 - t) + b * t
}

#[inline(always)]
pub fn bi_lerp<T>(c00: T, c01: T, c10: T, c11: T, dx: f64, dy: f64) -> T
where
    T: Copy + Arithmetic,
{
    let a = lerp(c00, c10, dx);
    let b = lerp(c01, c11, dx);

    lerp(a, b, dy)
}

#[inline(always)]
pub fn barycentric_interpolate<T>(w0: f64, w1: f64, w2: f64, v0: T, v1: T, v2: T) -> T
where
    T: Mul<f64, Output = T> + Add<Output = T>,
{
    v0 * w0 + v1 * w1 + v2 * w2
}

#[inline(always)]
pub fn perspective_interpolate<T>(
    bary: (f64, f64, f64),
    inv_d_lerped: f64,
    elements: (T, T, T),
) -> T
where
    T: Mul<f64, Output = T> + Add<Output = T> + Copy,
{
    let (w0, w1, w2) = bary;
    let (v0, v1, v2) = elements;
    let v_prime = barycentric_interpolate(w0, w1, w2, v0, v1, v2);

    v_prime * (1.0 / inv_d_lerped)
}
