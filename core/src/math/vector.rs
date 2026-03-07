use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Default, Debug, Clone, Copy)]
pub struct Vector2 {
	pub x: f32,
	pub y: f32,
}

impl From<Vector3> for Vector2 {
	#[inline(always)]
	fn from(v3: Vector3) -> Self {
		Vector2 { x: v3.x, y: v3.y }
	}
}

impl Vector2 {
	pub const UNIT_X: Vector2 = Vector2 { x: 1.0, y: 0.0 };
	pub const UNIT_Y: Vector2 = Vector2 { x: 0.0, y: 1.0 };
	pub const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };

	#[inline(always)]
	pub const fn new(x: f32, y: f32) -> Self {
		Vector2 { x, y }
	}

	#[inline(always)]
	pub fn magnitude(&self) -> f32 {
		(self.x * self.x + self.y * self.y).sqrt()
	}

	#[inline(always)]
	pub fn normalize(&self) -> Self {
		let mag = self.magnitude();
		Vector2 {
			x: self.x / mag,
			y: self.y / mag,
		}
	}

	#[inline(always)]
	pub fn xy<T>(&self) -> (T, T)
	where
		T: From<f32>,
	{
		(T::from(self.x), T::from(self.y))
	}
}

impl Div<f32> for Vector2 {
	type Output = Self;

	#[inline(always)]
	fn div(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x / rhs,
			y: self.y / rhs,
		}
	}
}

impl Mul<f32> for Vector2 {
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
		}
	}
}

impl Mul for Vector2 {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Vector2 {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
		}
	}
}

impl Add for Vector2 {
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl Sub for Vector2 {
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Vector3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vector3 {
	pub const UNIT_X: Vector3 = Vector3 {
		x: 1.0,
		y: 0.0,
		z: 0.0,
	};
	pub const UNIT_Y: Vector3 = Vector3 {
		x: 0.0,
		y: 1.0,
		z: 0.0,
	};
	pub const UNIT_Z: Vector3 = Vector3 {
		x: 0.0,
		y: 0.0,
		z: 1.0,
	};
	pub const ZERO: Vector3 = Vector3 {
		x: 0.0,
		y: 0.0,
		z: 0.0,
	};

	#[inline(always)]
	pub const fn splat(n: f32) -> Self {
		Self { x: n, y: n, z: n }
	}

	#[inline(always)]
	pub const fn new(x: f32, y: f32, z: f32) -> Self {
		Vector3 { x, y, z }
	}

	#[inline(always)]
	pub fn magnitude(&self) -> f32 {
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}

	#[inline(always)]
	pub fn normalize(&self) -> Self {
		let mag = self.magnitude();
		Vector3 {
			x: self.x / mag,
			y: self.y / mag,
			z: self.z / mag,
		}
	}

	#[inline(always)]
	pub fn self_normalize(&mut self) {
		let n = self.normalize();
		self.x = n.x;
		self.y = n.y;
		self.z = n.z;
	}

	#[inline(always)]
	pub fn dot(&self, other: &Vector3) -> f32 {
		self.x * other.x + self.y * other.y + self.z * other.z
	}

	#[inline(always)]
	pub fn cross(&self, other: &Vector3) -> Self {
		Vector3 {
			x: self.y * other.z - self.z * other.y,
			y: self.z * other.x - self.x * other.z,
			z: self.x * other.y - self.y * other.x,
		}
	}

	#[inline(always)]
	pub fn xy(&self) -> Vector2 {
		Vector2 {
			x: self.x,
			y: self.y,
		}
	}
}

impl Mul for Vector3 {
	type Output = Vector3;

	#[inline(always)]
	fn mul(self, _scalar: Self) -> Vector3 {
		self
	}
}

impl Mul<f32> for Vector3 {
	type Output = Vector3;

	#[inline(always)]
	fn mul(self, scalar: f32) -> Vector3 {
		Vector3 {
			x: self.x * scalar,
			y: self.y * scalar,
			z: self.z * scalar,
		}
	}
}

impl Sub for Vector3 {
	type Output = Vector3;

	#[inline(always)]
	fn sub(self, other: Vector3) -> Vector3 {
		Vector3 {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}
}

impl Add for Vector3 {
	type Output = Vector3;

	#[inline(always)]
	fn add(self, other: Vector3) -> Vector3 {
		Vector3 {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}
}

impl AddAssign for Vector3 {
	#[inline(always)]
	fn add_assign(&mut self, other: Vector3) {
		self.x += other.x;
		self.y += other.y;
		self.z += other.z;
	}
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Vector4 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}

impl Vector4 {
	#[inline(always)]
	pub fn xyz(&self) -> Vector3 {
		Vector3 {
			x: self.x,
			y: self.y,
			z: self.z,
		}
	}
}
impl Div<f32> for Vector4 {
	type Output = Vector4;

	#[inline(always)]
	fn div(self, scalar: f32) -> Vector4 {
		Vector4 {
			x: self.x / scalar,
			y: self.y / scalar,
			z: self.z / scalar,
			w: self.w / scalar,
		}
	}
}

impl Mul<f32> for Vector4 {
	type Output = Vector4;

	#[inline(always)]
	fn mul(self, rhs: f32) -> Self::Output {
		Vector4 {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
			w: self.w * rhs,
		}
	}
}

impl Vector4 {
	#[inline(always)]
	pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
		Vector4 { x, y, z, w }
	}
}

impl From<(Vector3, f32)> for Vector4 {
	#[inline(always)]
	fn from((v3, w): (Vector3, f32)) -> Self {
		Vector4 {
			x: v3.x,
			y: v3.y,
			z: v3.z,
			w,
		}
	}
}
