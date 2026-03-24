use {
	crate::{
		geometry::{Indices, Mesh, UV},
		math::Vector3,
	},
	core::f32,
};

pub enum Shapes {
	Sphere,
	Plane,
	Cube,
	Trapezoid,
	Pyramid,
	Cone,
}

pub fn generate_sphere(radius: f32, sectors: usize, rings: usize) -> Mesh {
	let mut positions = Vec::new();
	let mut uvs = Vec::new();
	let mut normals = Vec::new();

	for r in 0..=rings {
		let v = r as f32 / rings as f32;
		let phi = v * f32::consts::PI;

		for s in 0..=sectors {
			let u = s as f32 / sectors as f32;
			let theta = u * f32::consts::PI * 2.0;

			let x = radius * phi.sin() * theta.cos();
			let y = radius * phi.cos();
			let z = radius * phi.sin() * theta.sin();

			let position = Vector3::new(x, y, z);
			let normal = position.normalize();
			let uv = UV::new(u, v);

			positions.push(position);
			normals.push(normal);
			uvs.push(uv);
		}
	}

	let mut indices = Indices::default();

	let push_indices =
		|indices: &mut Indices, v0: usize, v1: usize, v2: usize| {
			indices.push_v_index(v0);
			indices.push_v_index(v1);
			indices.push_v_index(v2);

			indices.push_n_index(v0);
			indices.push_n_index(v1);
			indices.push_n_index(v2);

			indices.push_uv_index(v0);
			indices.push_uv_index(v1);
			indices.push_uv_index(v2);
		};

	for r in 0..rings {
		for s in 0..sectors {
			let current = r * (sectors + 1) + s;
			let next = current + sectors + 1;

			// Triangle 1
			push_indices(&mut indices, current, current + 1, next);
			// Triangle 2
			push_indices(&mut indices, current + 1, next + 1, next);
		}
	}

	Mesh::new(positions, uvs, indices, normals)
}

pub fn generate_frustum(
	bottom_w: f32,
	bottom_d: f32,
	top_w: f32,
	top_d: f32,
	height: f32,
) -> Mesh {
	let mut positions = Vec::new();
	let mut uvs = Vec::new();
	let mut normals = Vec::new();

	let mut indices = Indices::default();

	let hw_b = bottom_w / 2.0;
	let hd_b = bottom_d / 2.0;
	let hw_t = top_w / 2.0;
	let hd_t = top_d / 2.0;
	let y_b = -height / 2.0;
	let y_t = height / 2.0;

	let mut add_face = |p0: Vector3, p1: Vector3, p2: Vector3, p3: Vector3| {
		let normal = (p1 - p0).cross(&(p2 - p0)).normalize();
		let base = positions.len();

		positions.extend_from_slice(&[p0, p1, p2, p3]);
		normals.extend_from_slice(&[normal, normal, normal, normal]);
		uvs.extend_from_slice(&[
			UV::new(0.0, 0.0),
			UV::new(1.0, 0.0),
			UV::new(1.0, 1.0),
			UV::new(0.0, 1.0),
		]);

		for i in [0usize, 1, 2, 0, 2, 3] {
			let index = i + base;
			indices.push_v_index(index);
			indices.push_n_index(index);
			indices.push_uv_index(index);
		}
	};

	// Define the 8 corners
	let b_bl = Vector3::new(-hw_b, y_b, -hd_b); // Bottom Back Left
	let b_br = Vector3::new(hw_b, y_b, -hd_b); // Bottom Back Right
	let b_fl = Vector3::new(-hw_b, y_b, hd_b); // Bottom Front Left
	let b_fr = Vector3::new(hw_b, y_b, hd_b); // Bottom Front Right

	let t_bl = Vector3::new(-hw_t, y_t, -hd_t); // Top Back Left
	let t_br = Vector3::new(hw_t, y_t, -hd_t); // Top Back Right
	let t_fl = Vector3::new(-hw_t, y_t, hd_t); // Top Front Left
	let t_fr = Vector3::new(hw_t, y_t, hd_t); // Top Front Right

	// Add 6 faces (Order matters for winding/culling! Counter-clockwise)
	add_face(b_fl, b_fr, t_fr, t_fl); // Front
	add_face(b_br, b_bl, t_bl, t_br); // Back
	add_face(b_bl, b_fl, t_fl, t_bl); // Left
	add_face(b_fr, b_br, t_br, t_fr); // Right
	add_face(t_fl, t_fr, t_br, t_bl); // Top
	add_face(b_bl, b_br, b_fr, b_fl); // Bottom

	Mesh::new(positions, uvs, indices, normals)
}

pub fn generate_plane(width: f32, depth: f32) -> Mesh {
	let hw = width / 2.0;
	let hd = depth / 2.0;
	let normal = Vector3::new(0.0, 1.0, 0.0);

	let mut indices = Indices::default();

	let positions = vec![
		Vector3::new(-hw, 0.0, -hd),
		Vector3::new(hw, 0.0, -hd),
		Vector3::new(hw, 0.0, hd),
		Vector3::new(-hw, 0.0, hd),
	];

	let normals = vec![normal, normal, normal, normal];

	let uvs = vec![
		UV::new(0.0, 0.0),
		UV::new(1.0, 0.0),
		UV::new(1.0, 1.0),
		UV::new(0.0, 1.0),
	];

	for i in [0usize, 2, 1, 0, 3, 2] {
		let index = i;
		indices.push_v_index(index);
		indices.push_n_index(index);
		indices.push_uv_index(index);
	}

	Mesh::new(positions, uvs, indices, normals)
}

pub fn generate_cube(size: f32) -> Mesh {
	let mut positions = Vec::new();
	let mut uvs = Vec::new();
	let mut normals = Vec::new();
	let mut indices = Indices::default();

	let h = size * 0.5;

	let mut add_face = |p0: Vector3, p1: Vector3, p2: Vector3, p3: Vector3| {
		let normal = (p1 - p0).cross(&(p2 - p0)).normalize();
		let base = positions.len();

		positions.extend_from_slice(&[p0, p1, p2, p3]);
		normals.extend_from_slice(&[normal, normal, normal, normal]);
		uvs.extend_from_slice(&[
			UV::new(0.0, 0.0),
			UV::new(1.0, 0.0),
			UV::new(1.0, 1.0),
			UV::new(0.0, 1.0),
		]);

		for i in [0usize, 1, 2, 0, 2, 3] {
			let idx = base + i;
			indices.push_v_index(idx);
			indices.push_n_index(idx);
			indices.push_uv_index(idx);
		}
	};

	let b_bl = Vector3::new(-h, -h, -h);
	let b_br = Vector3::new(h, -h, -h);
	let b_fl = Vector3::new(-h, -h, h);
	let b_fr = Vector3::new(h, -h, h);

	let t_bl = Vector3::new(-h, h, -h);
	let t_br = Vector3::new(h, h, -h);
	let t_fl = Vector3::new(-h, h, h);
	let t_fr = Vector3::new(h, h, h);

	add_face(b_fl, b_fr, t_fr, t_fl); // Front  (+Z)
	add_face(b_br, b_bl, t_bl, t_br); // Back   (-Z)
	add_face(b_bl, b_fl, t_fl, t_bl); // Left   (-X)
	add_face(b_fr, b_br, t_br, t_fr); // Right  (+X)
	add_face(t_fl, t_fr, t_br, t_bl); // Top    (+Y)
	add_face(b_bl, b_br, b_fr, b_fl); // Bottom (-Y)

	Mesh::new(positions, uvs, indices, normals)
}

pub fn generate_pyramid(base_w: f32, base_d: f32, height: f32) -> Mesh {
	let mut positions = Vec::new();
	let mut uvs = Vec::new();
	let mut normals = Vec::new();
	let mut indices = Indices::default();

	let hw = base_w * 0.5;
	let hd = base_d * 0.5;
	let y_b = -height * 0.5;
	let y_t = height * 0.5;

	let apex = Vector3::new(0.0, y_t, 0.0);

	let b_bl = Vector3::new(-hw, y_b, -hd);
	let b_br = Vector3::new(hw, y_b, -hd);
	let b_fl = Vector3::new(-hw, y_b, hd);
	let b_fr = Vector3::new(hw, y_b, hd);

	let add_face = |positions: &mut Vec<Vector3>,
	                normals: &mut Vec<Vector3>,
	                uvs: &mut Vec<UV>,
	                indices: &mut Indices,
	                p0: Vector3,
	                p1: Vector3,
	                p2: Vector3,
	                p3: Vector3| {
		let normal = (p1 - p0).cross(&(p2 - p0)).normalize();
		let base = positions.len();

		positions.extend_from_slice(&[p0, p1, p2, p3]);
		normals.extend_from_slice(&[normal, normal, normal, normal]);
		uvs.extend_from_slice(&[
			UV::new(0.0, 0.0),
			UV::new(1.0, 0.0),
			UV::new(1.0, 1.0),
			UV::new(0.0, 1.0),
		]);

		for i in [0usize, 1, 2, 0, 2, 3] {
			let idx = base + i;
			indices.push_v_index(idx);
			indices.push_n_index(idx);
			indices.push_uv_index(idx);
		}
	};

	let add_tri = |positions: &mut Vec<Vector3>,
	               normals: &mut Vec<Vector3>,
	               uvs: &mut Vec<UV>,
	               indices: &mut Indices,
	               p0: Vector3,
	               p1: Vector3,
	               p2: Vector3| {
		let normal = (p1 - p0).cross(&(p2 - p0)).normalize();
		let base = positions.len();

		positions.extend_from_slice(&[p0, p1, p2]);
		normals.extend_from_slice(&[normal, normal, normal]);
		uvs.extend_from_slice(&[
			UV::new(0.0, 0.0),
			UV::new(1.0, 0.0),
			UV::new(0.5, 1.0),
		]);

		for i in [0usize, 1, 2] {
			let idx = base + i;
			indices.push_v_index(idx);
			indices.push_n_index(idx);
			indices.push_uv_index(idx);
		}
	};

	// Base (outward = -Y)
	add_face(
		&mut positions,
		&mut normals,
		&mut uvs,
		&mut indices,
		b_bl,
		b_br,
		b_fr,
		b_fl,
	);

	// Sides (CCW, outward)
	add_tri(
		&mut positions,
		&mut normals,
		&mut uvs,
		&mut indices,
		b_fl,
		b_fr,
		apex,
	); // Front
	add_tri(
		&mut positions,
		&mut normals,
		&mut uvs,
		&mut indices,
		b_fr,
		b_br,
		apex,
	); // Right
	add_tri(
		&mut positions,
		&mut normals,
		&mut uvs,
		&mut indices,
		b_br,
		b_bl,
		apex,
	); // Back
	add_tri(
		&mut positions,
		&mut normals,
		&mut uvs,
		&mut indices,
		b_bl,
		b_fl,
		apex,
	); // Left

	Mesh::new(positions, uvs, indices, normals)
}

pub fn generate_cone(radius: f32, height: f32, sectors: usize) -> Mesh {
	let mut positions = Vec::new();
	let mut uvs = Vec::new();
	let mut normals = Vec::new();
	let mut indices = Indices::default();

	let y_b = -height * 0.5;
	let y_t = height * 0.5;
	let apex = Vector3::new(0.0, y_t, 0.0);
	let center = Vector3::new(0.0, y_b, 0.0);

	let two_pi = f32::consts::PI * 2.0;
	let inv_h = if height.abs() < 1e-6 {
		0.0
	} else {
		radius / height
	};

	let mut push_tri = |i0: usize, i1: usize, i2: usize| {
		for idx in [i0, i1, i2] {
			indices.push_v_index(idx);
			indices.push_n_index(idx);
			indices.push_uv_index(idx);
		}
	};

	// Side ring vertices (shared) with smooth normals
	for s in 0..=sectors {
		let u = s as f32 / sectors as f32;
		let t = u * two_pi;
		let (ct, st) = (t.cos(), t.sin());

		let x = radius * ct;
		let z = radius * st;

		positions.push(Vector3::new(x, y_b, z));
		normals.push(Vector3::new(ct, inv_h, st).normalize());
		uvs.push(UV::new(u, 0.0));
	}

	let apex_index = positions.len();
	positions.push(apex);
	normals.push(Vector3::new(0.0, 1.0, 0.0));
	uvs.push(UV::new(0.5, 1.0));

	for s in 0..sectors {
		let i0 = s;
		let i1 = s + 1;
		let i2 = apex_index;
		// CCW outward
		push_tri(i1, i0, i2);
	}

	// Base ring vertices (separate so base stays flat)
	let base_start = positions.len();
	for s in 0..=sectors {
		let u = s as f32 / sectors as f32;
		let t = u * two_pi;
		let (ct, st) = (t.cos(), t.sin());

		let x = radius * ct;
		let z = radius * st;

		positions.push(Vector3::new(x, y_b, z));
		normals.push(Vector3::new(0.0, -1.0, 0.0));
		uvs.push(UV::new(0.5 + x / (2.0 * radius), 0.5 + z / (2.0 * radius)));
	}

	let center_index = positions.len();
	positions.push(center);
	normals.push(Vector3::new(0.0, -1.0, 0.0));
	uvs.push(UV::new(0.5, 0.5));

	for s in 0..sectors {
		let i0 = base_start + s;
		let i1 = base_start + s + 1;
		let i2 = center_index;
		// CCW outward for bottom (-Y)
		push_tri(i0, i1, i2);
	}

	Mesh::new(positions, uvs, indices, normals)
}
