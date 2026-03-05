use {
	pcore::{
		geometry::{BiTangent, Normal, Tangent, UV, VertexAttributes},
		math::{Vector3, Vector4},
	},
	std::ops::{Add, Mul},
};

#[derive(Debug, Clone, Copy)]
pub struct VertexIn {
	pub attributes: VertexAttributes,
	pub face_normal: Vector3,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Varyings {
	pub uv: UV,
	pub normal: Normal,
	pub tangent: Tangent,
	pub bi_tangent: BiTangent,
	pub world_pos: Vector3,
	pub intensity: f64,
}

impl Mul<f64> for Varyings {
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: f64) -> Self::Output {
		Self {
			uv: self.uv * rhs,
			normal: self.normal * rhs,
			tangent: self.tangent * rhs,
			bi_tangent: self.bi_tangent * rhs,
			world_pos: self.world_pos * rhs,
			intensity: self.intensity * rhs,
		}
	}
}

impl Add for Varyings {
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: Self) -> Self::Output {
		Self {
			uv: self.uv + rhs.uv,
			normal: self.normal + rhs.normal,
			tangent: self.tangent + rhs.tangent,
			bi_tangent: self.bi_tangent + rhs.bi_tangent,
			world_pos: self.world_pos + rhs.world_pos,
			intensity: self.intensity + rhs.intensity,
		}
	}
}

#[derive(Default, Debug, Clone, Copy)]
pub struct VertexOut {
	pub clip: Vector4,
	pub vary: Varyings,
}
