use {
	pcore::{
		geometry::{BiTangent, Normal, Tangent, UV, VertexAttributes},
		math::{Gradient, Vector2, Vector3, Vector4},
	},
	std::ops::{Add, Mul, Sub},
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
	pub intensity: f32,
}

impl Mul<f32> for Varyings {
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: f32) -> Self::Output {
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

impl Mul for Varyings {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Varyings {
			uv: self.uv * rhs.uv,
			normal: self.normal * rhs.normal,
			tangent: self.tangent * rhs.tangent,
			bi_tangent: self.bi_tangent * rhs.bi_tangent,
			world_pos: self.world_pos * rhs.world_pos,
			intensity: self.intensity * rhs.intensity,
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

impl Sub for Varyings {
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			uv: self.uv - rhs.uv,
			normal: self.normal - rhs.normal,
			tangent: self.tangent - rhs.tangent,
			bi_tangent: self.bi_tangent - rhs.bi_tangent,
			world_pos: self.world_pos - rhs.world_pos,
			intensity: self.intensity - rhs.intensity,
		}
	}
}

#[derive(Default, Debug, Clone, Copy)]
pub struct VertexOut {
	pub clip: Vector4,
	pub vary: Varyings,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct GVaryings {
	pub uv: Gradient<UV>,
	pub normal: Gradient<Normal>,
	pub tangent: Gradient<Tangent>,
	pub bi_tangent: Gradient<BiTangent>,
	pub world_pos: Gradient<Vector3>,
}

impl GVaryings {
	pub fn from_varyings(
		v: [Varyings; 3],
		s: [Vector2; 3],
		inv_det: f32,
	) -> Self {
		Self {
			uv: Gradient::new([v[0].uv, v[1].uv, v[2].uv], s, inv_det),
			normal: Gradient::new(
				[v[0].normal, v[1].normal, v[2].normal],
				s,
				inv_det,
			),
			tangent: Gradient::new(
				[v[0].tangent, v[1].tangent, v[2].tangent],
				s,
				inv_det,
			),
			bi_tangent: Gradient::new(
				[v[0].bi_tangent, v[1].bi_tangent, v[2].bi_tangent],
				s,
				inv_det,
			),
			world_pos: Gradient::new(
				[v[0].world_pos, v[1].world_pos, v[2].world_pos],
				s,
				inv_det,
			),
		}
	}
}
