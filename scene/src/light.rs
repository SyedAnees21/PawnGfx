// use {crate::color::Color, pcore::math::Vector3};
use pcore::{color::Color, math::Vector3};

// pub enum Light {
// 	Directional {
// 		position: Vector3,
// 		ambient: f32,
// 		color: Color
// 	}
// }

pub struct Light {
	pub position: Vector3,
	pub color: Color,
	pub ambient: f32,
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
