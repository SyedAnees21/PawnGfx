use crate::math::{Arithmetic, Vector2};

#[derive(Default, Debug, Clone, Copy)]
pub struct Gradient<T> {
	pub a: T,
	pub da_dx: T,
	pub da_dy: T,
}

impl<T> Gradient<T>
where
	T: Arithmetic,
{
	#[inline(always)]
	pub fn new(v: [T; 3], screen: [Vector2; 3], inv_det: f32) -> Self {
		// Edge-function aligned basis (pivot at v0).
		// This aligns with area = edge(v0, v1, v2):
		// area = (x2 - x0) * (y1 - y0) - (y2 - y0) * (x1 - x0)
		let dx10 = screen[1].x - screen[0].x;
		let dy10 = screen[1].y - screen[0].y;
		let dx20 = screen[2].x - screen[0].x;
		let dy20 = screen[2].y - screen[0].y;

		let a10 = v[1] - v[0];
		let a20 = v[2] - v[0];

		// The partial derivatives (Step values), using inv_det = 1 / edge(v0, v1,
		// v2)
		let step_x = (a20 * dy10 - a10 * dy20) * inv_det;
		let step_y = (a10 * dx20 - a20 * dx10) * inv_det;

		Self {
			a: v[0],
			da_dx: step_x,
			da_dy: step_y,
		}
	}

	/// Calculate the value at a specific screen offset from V0
	#[inline(always)]
	pub fn sample_at(&self, dx: f32, dy: f32) -> T {
		self.a + (self.da_dx * dx) + (self.da_dy * dy)
	}

	#[inline(always)]
	pub fn step_x(&self, val: &mut T) {
		*val = *val + self.da_dx;
	}

	#[inline(always)]
	pub fn step_y(&self, val: &mut T) {
		*val = *val + self.da_dy;
	}
}
