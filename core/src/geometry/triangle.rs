use crate::{
	geometry::{Mesh, VertexAttributes},
	math::Vector2,
};

pub struct Triangles<'a> {
	pub mesh: &'a Mesh,
	pub counter: usize,
}

impl Iterator for Triangles<'_> {
	type Item = [VertexAttributes; 3];

	fn next(&mut self) -> Option<Self::Item> {
		if self.counter + 2 >= self.mesh.indices.len() {
			return None;
		}

		let v_attributes = [0, 1, 2].map(|i| {
			let index = self.counter + i;

			let v_id = self.mesh.indices.v[index];
			let n_id = self.mesh.indices.n[index];
			let uv_id = self.mesh.indices.uv[index];

			let v = self.mesh.vertices[v_id];
			let n = self.mesh.normals[n_id];
			let uv = self.mesh.uv[uv_id];

			let tangent = self.mesh.tangents[v_id];
			let bi_tangent = self.mesh.bi_tangents[v_id];

			VertexAttributes {
				position: v,
				normal: n,
				uv,
				tangent,
				bi_tangent,
			}
		});

		self.counter += 3;
		Some(v_attributes)
	}
}

impl<'a> Triangles<'a> {
	pub fn new(mesh: &'a Mesh) -> Self {
		Self { mesh, counter: 0 }
	}
}

pub struct IncEdge {
	e0: Edge,
	e1: Edge,
	e2: Edge,
}

impl IncEdge {
	#[inline(always)]
	pub fn new(
		v0: Vector2,
		v1: Vector2,
		v2: Vector2,
		inv_det: Option<f32>,
	) -> Self {
		Self {
			e0: Edge::new(v1, v2, inv_det),
			e1: Edge::new(v2, v0, inv_det),
			e2: Edge::new(v0, v1, inv_det),
		}
	}

	#[inline(always)]
	pub fn weights(&self, x: f32, y: f32) -> (f32, f32, f32) {
		(self.e0.eval(x, y), self.e1.eval(x, y), self.e2.eval(x, y))
	}

	#[inline(always)]
	pub fn step_x(&self, w0: f32, w1: f32, w2: f32) -> (f32, f32, f32) {
		(self.e0.step_x(w0), self.e1.step_x(w1), self.e2.step_x(w2))
	}

	#[inline(always)]
	pub fn step_y(&self, w0: f32, w1: f32, w2: f32) -> (f32, f32, f32) {
		(self.e0.step_y(w0), self.e1.step_y(w1), self.e2.step_y(w2))
	}
}

pub struct Edge {
	pub a: f32,
	pub b: f32,
	pub c: f32,
}

impl Edge {
	#[inline(always)]
	pub fn new(v0: Vector2, v1: Vector2, inv_det: Option<f32>) -> Self {
		let det = inv_det.unwrap_or(1.0);
		Self {
			a: (v1.y - v0.y) * det,
			b: (v0.x - v1.x) * det,
			c: (v1.x * v0.y - v0.x * v1.y) * det,
		}
	}

	#[inline(always)]
	pub fn eval(&self, x: f32, y: f32) -> f32 {
		self.a * x + self.b * y + self.c
	}

	#[inline(always)]
	pub fn step_x(&self, val: f32) -> f32 {
		val + self.a
	}

	#[inline(always)]
	pub fn step_y(&self, val: f32) -> f32 {
		val + self.b
	}
}
