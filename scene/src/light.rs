use pcore::math::Vector3;
use crate::color::Color;

pub struct Light {
	pub position: Vector3,
	pub color: Color,
	pub ambient: f64,
}

impl Default for Light {
	fn default() -> Self {
		Self {
			color: Color::WHITE,
			position: Vector3::new(1.0, 1.0, 2.0),
			ambient: 0.5,
		}
	}
}

impl Light {
	pub fn direction(&self) -> Vector3 {
		self.position.normalize()
	}
}
