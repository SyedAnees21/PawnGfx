use crate::{
	geometry::{
		BiTangent,
		Idx,
		NIdx,
		Normal,
		Normals,
		TIdx,
		Tangent,
		Triangles,
		UV,
		VIdx,
		Vertex,
		Vertices,
	},
	math::Vector3,
};

#[derive(Debug)]
pub struct Indices {
	pub v: Vec<Idx>,
	pub n: Vec<Idx>,
	pub uv: Vec<Idx>,
}

impl Default for Indices {
	fn default() -> Self {
		Self {
			v: vec![],
			n: vec![],
			uv: vec![],
		}
	}
}

impl Indices {
	pub fn len(&self) -> usize {
		self.v.len()
	}

	pub fn index(&self, index: Idx) -> (VIdx, NIdx, TIdx) {
		if index >= self.len() {
			return (0, 0, 0);
		}
		(self.v[index], self.n[index], self.uv[index])
	}

	pub fn push_v_index(&mut self, idx: Idx) {
		self.v.push(idx);
	}

	pub fn push_n_index(&mut self, idx: Idx) {
		self.n.push(idx);
	}

	pub fn push_uv_index(&mut self, idx: Idx) {
		self.uv.push(idx);
	}
}

#[derive(Debug)]
pub struct Mesh {
	pub vertices: Vertices,
	pub normals: Normals,
	pub uv: Vec<UV>,
	pub tangents: Vec<Tangent>,
	pub bi_tangents: Vec<BiTangent>,
	pub indices: Indices,
}

impl Default for Mesh {
	fn default() -> Self {
		Self {
			vertices: vec![],
			normals: vec![],
			uv: vec![],
			tangents: vec![],
			bi_tangents: vec![],
			indices: Indices::default(),
		}
	}
}

impl Mesh {
	pub fn new(
		vertices: Vertices,
		mut uv: Vec<UV>,
		mut indices: Indices,
		mut vnormals: Normals,
	) -> Self {
		let (tangents, bi_tangents) =
			Self::bake_mesh(&vertices, &mut indices, &mut uv, &mut vnormals);

		Self {
			vertices,
			indices,
			uv,
			tangents,
			bi_tangents,
			normals: vnormals,
		}
	}

	pub fn from_vertices_faces(vertices: Vertices, faces: Vec<Idx>) -> Self {
		Self {
			vertices,
			indices: Indices {
				v: faces,
				..Default::default()
			},
			..Default::default()
		}
	}

	pub fn vertex_count(&self) -> usize {
		self.vertices.len()
	}

	pub fn index_count(&self) -> usize {
		self.indices.len()
	}

	pub fn get_indices(&self, index: Idx) -> (VIdx, NIdx, TIdx) {
		self.indices.index(index)
	}

	pub fn iter_triangles(&self) -> Triangles {
		Triangles::new(self)
	}

	pub fn has_normals(&self) -> bool {
		!self.normals.is_empty()
	}

	pub fn has_uv(&self) -> bool {
		!self.uv.is_empty()
	}

	fn bake_mesh(
		vertices: &Vertices,
		indices: &mut Indices,
		uv: &mut Vec<UV>,
		normals: &mut Normals,
	) -> (Vec<Tangent>, Vec<BiTangent>) {
		if normals.is_empty() {
			Self::bake_normals(vertices, indices, normals);
		}

		let tangents = if !uv.is_empty() {
			Self::bake_tangents(vertices, uv, indices)
		} else {
			(vec![], vec![])
		};

		tangents
	}

	fn bake_normals(
		vertices: &Vertices,
		indices: &mut Indices,
		normals: &mut Normals,
	) {
		normals.resize(vertices.len(), Normal::default());
		indices.n.resize(indices.v.len(), 0);

		let count = indices.v.len() / 3;

		for i in 0..count {
			let i_0 = i * 3;
			let i_1 = i * 3 + 1;
			let i_2 = i * 3 + 2;

			let id0 = indices.v[i_0];
			let id1 = indices.v[i_1];
			let id2 = indices.v[i_2];

			let v0 = vertices[id0];
			let v1 = vertices[id1];
			let v2 = vertices[id2];

			let f_normal = (v1 - v0).cross(&(v2 - v0));

			indices.n[i_0] = id0;
			indices.n[i_1] = id1;
			indices.n[i_2] = id2;

			normals[id0] = normals[id0] + f_normal;
			normals[id1] = normals[id1] + f_normal;
			normals[id2] = normals[id2] + f_normal;
		}

		normals.iter_mut().for_each(|n| *n = n.normalize());
	}

	fn bake_tangents(
		vertices: &Vertices,
		uv: &Vec<UV>,
		indices: &Indices,
	) -> (Vec<Tangent>, Vec<BiTangent>) {
		let size: usize = indices.v.len();
		let mut tangents = vec![Vector3::default(); size];
		let mut bi_tangents = vec![Vector3::default(); size];

		let count = size / 3;

		for i in 0..count {
			let i_0 = i * 3;
			let i_1 = i * 3 + 1;
			let i_2 = i * 3 + 2;

			let v_id0 = indices.v[i_0];
			let v_id1 = indices.v[i_1];
			let v_id2 = indices.v[i_2];

			let uv_id0 = indices.uv[i_0];
			let uv_id1 = indices.uv[i_1];
			let uv_id2 = indices.uv[i_2];

			let v0 = vertices[v_id0];
			let v1 = vertices[v_id1];
			let v2 = vertices[v_id2];

			let uv_0 = uv[uv_id0];
			let uv_1 = uv[uv_id1];
			let uv_2 = uv[uv_id2];

			let v_e1 = v1 - v0;
			let v_e2 = v2 - v0;

			let uv_e1 = uv_1 - uv_0;
			let uv_e2 = uv_2 - uv_0;

			let f = 1.0 / (uv_e1.x * uv_e2.y - uv_e2.x * uv_e1.y);

			let tangent = (v_e1 * uv_e2.y - v_e2 * uv_e1.y) * f;
			let bi_tangent = (v_e2 * uv_e1.x - v_e1 * uv_e2.x) * f;

			tangents[v_id0] += tangent;
			tangents[v_id1] += tangent;
			tangents[v_id2] += tangent;

			bi_tangents[v_id0] += bi_tangent;
			bi_tangents[v_id1] += bi_tangent;
			bi_tangents[v_id2] += bi_tangent;
		}

		for (t, b) in tangents.iter_mut().zip(bi_tangents.iter_mut()) {
			t.self_normalize();
			b.self_normalize();
		}

		(tangents, bi_tangents)
	}
}

pub struct IterVertices<'a> {
	mesh: &'a Mesh,
	counter: usize,
}

impl Iterator for IterVertices<'_> {
	type Item = (Vertex, Normal, UV);

	fn next(&mut self) -> Option<Self::Item> {
		if self.mesh.indices.len() <= self.counter {
			return None;
		}

		let (vi, ni, uvi) = self.mesh.get_indices(self.counter);
		let v = self.mesh.vertices[vi];
		let n = self.mesh.normals[ni];
		let uv = self.mesh.uv[uvi];

		Some((v, n, uv))
	}
}

pub struct IterNormals<'a> {
	mesh: &'a Mesh,
	counter: usize,
}

impl Iterator for IterNormals<'_> {
	type Item = Normal;

	fn next(&mut self) -> Option<Self::Item> {
		if self.mesh.indices.n.len() == 0
			|| self.mesh.indices.n.len() <= self.counter
		{
			return None;
		}

		let idx = self.counter;
		self.counter += 1;

		Some(self.mesh.normals[idx])
	}
}

pub struct IterUV<'a> {
	mesh: &'a Mesh,
	counter: usize,
}

impl Iterator for IterUV<'_> {
	type Item = UV;

	fn next(&mut self) -> Option<Self::Item> {
		if self.mesh.indices.uv.len() == 0
			|| self.mesh.indices.uv.len() <= self.counter
		{
			return None;
		}

		let idx = self.counter;
		self.counter += 1;

		Some(self.mesh.uv[idx])
	}
}
