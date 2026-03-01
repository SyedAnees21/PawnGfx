use crate::geometry::{Mesh, VertexAttributes};

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

		// let mut v_attributes =
		// [VertexAttributes::default(); 3];

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

		// for i in 0..3 {
		// 	let index = self.counter + i;

		// 	let v_id = self.mesh.indices.v[index];
		// 	let n_id = self.mesh.indices.n[index];
		// 	let uv_id = self.mesh.indices.uv[index];

		// 	let v = self.mesh.vertices[v_id];
		// 	let n = self.mesh.normals[n_id];
		// 	let uv = self.mesh.uv[uv_id];

		// 	let tangent = self.mesh.tangents[v_id];
		// 	let bi_tangent =
		// self.mesh.bi_tangents[v_id];

		// 	v_attributes[i].set_position(v);
		// 	v_attributes[i].set_normal(n);
		// 	v_attributes[i].set_uv(uv);
		// 	v_attributes[i].set_tangent(tangent);
		// 	v_attributes[i].
		// set_bi_tangent(bi_tangent); }

		self.counter += 3;
		Some(v_attributes)
	}
}

impl<'a> Triangles<'a> {
	pub fn new(mesh: &'a Mesh) -> Self {
		Self { mesh, counter: 0 }
	}
}
