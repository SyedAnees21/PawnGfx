use std::ops::{Add, Mul, Sub};

mod gradient;
mod interpolate;
mod matrices;
mod vector;

pub use {interpolate::*, matrices::*, vector::*};

pub trait Arithmetic:
	Add<Output = Self>
	+ Sub<Output = Self>
	+ Mul<Output = Self>
	+ Mul<f32, Output = Self>
	+ Copy
where
	Self: Sized,
{
}

impl<T> Arithmetic for T where
	T: Add<Output = T>
		+ Sub<Output = T>
		+ Mul<Output = T>
		+ Mul<f32, Output = T>
		+ Copy
		+ Sized
{
}
