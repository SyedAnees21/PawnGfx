use std::ops::{Add, Mul, Sub};

mod interpolate;
mod matrices;
mod vector;

pub use {interpolate::*, matrices::*, vector::*};

pub trait Arithmetic:
	Add<Output = Self>
	+ Sub<Output = Self>
	+ Mul<Output = Self>
	+ Mul<f64, Output = Self>
where
	Self: Sized,
{
}

impl<T> Arithmetic for T where
	T: Add<Output = T>
		+ Sub<Output = T>
		+ Mul<Output = T>
		+ Mul<f64, Output = T>
		+ Sized
{
}
