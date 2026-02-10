use std::ops::{Add, Mul, Sub};

#[inline(always)]
pub fn lerp<T>(a: T, b: T, t: f64) -> T
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<f64, Output = T>,
{
    a * (1.0 - t) + b * t
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
    inv_depths: (f64, f64, f64),
    elements: (T, T, T),
) -> T
where
    T: Mul<f64, Output = T> + Add<Output = T> + Copy,
{
    let (w0, w1, w2) = bary;
    let (inv_w0, inv_w1, inv_w2) = inv_depths;
    let (v0, v1, v2) = elements;

    let v_prime = barycentric_interpolate(w0, w1, w2, v0, v1, v2);
    let inv_d_lerped = barycentric_interpolate(w0, w1, w2, inv_w0, inv_w1, inv_w2);

    v_prime * (1.0 / inv_d_lerped)
}
