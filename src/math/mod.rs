mod matrices;
mod vector;
use std::ops::{Add, Mul, Sub};

pub use matrices::*;
pub use vector::*;

pub fn lerp<T>(a: T, b: T, t: f64) -> T
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<f64, Output = T>,
{
    a * (1.0 - t) + b * t
}
