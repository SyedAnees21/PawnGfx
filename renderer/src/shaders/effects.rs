use {
	crate::shaders::{FS, GVaryings, VS, Varyings, VertexIn, VertexOut},
	pcore::{
		color::Color,
		math::{self, Matrix3, Vector4},
	},
	pscene::{
		// color::Color,
		texture::TextureSampler,
	},
};

fn shadow_factor(
	input: &Varyings,
	uniforms: &super::uniform::GlobalUniforms,
	normal: pcore::math::Vector3,
) -> f32 {
	let shadow = uniforms.shadow;
	if !shadow.enabled || shadow.map_size == 0 || shadow.depth_ptr.is_null() {
		return 1.0;
	}

	let lp = shadow.light_vp * Vector4::from((input.world_pos, 1.0));
	if lp.w.abs() < 1e-6 {
		return 1.0;
	}

	let ndc = lp * (1.0 / lp.w);

	if ndc.x < -1.0
		|| ndc.x > 1.0
		|| ndc.y < -1.0
		|| ndc.y > 1.0
		|| ndc.z < -1.0
		|| ndc.z > 1.0
	{
		return 1.0;
	}

	let light_dir = uniforms.light.direction;
	let n = normal.normalize();
	let ndotl = n.dot(&light_dir).max(0.0);
	if ndotl <= 0.0 {
		return 1.0;
	}
	let slope = 1.0 - ndotl;
	let bias = shadow.bias * (1.0 + 6.0 * slope);
	let current = ndc.z - bias;

	let size = shadow.map_size as i32;
	let u = (ndc.x * 0.5 + 0.5) * (size as f32 - 1.0);
	let v = (1.0 - (ndc.y * 0.5 + 0.5)) * (size as f32 - 1.0);

	let x0 = u.floor() as i32;
	let y0 = v.floor() as i32;
	let x1 = (x0 + 1).min(size - 1);
	let y1 = (y0 + 1).min(size - 1);

	let fx = u - x0 as f32;
	let fy = v - y0 as f32;

	let x0 = x0.clamp(0, size - 1) as usize;
	let y0 = y0.clamp(0, size - 1) as usize;
	let x1 = x1.clamp(0, size - 1) as usize;
	let y1 = y1.clamp(0, size - 1) as usize;

	let idx00 = y0 * shadow.map_size as usize + x0;
	let idx10 = y0 * shadow.map_size as usize + x1;
	let idx01 = y1 * shadow.map_size as usize + x0;
	let idx11 = y1 * shadow.map_size as usize + x1;

	let d00 = unsafe { *shadow.depth_ptr.add(idx00) };
	let d10 = unsafe { *shadow.depth_ptr.add(idx10) };
	let d01 = unsafe { *shadow.depth_ptr.add(idx01) };
	let d11 = unsafe { *shadow.depth_ptr.add(idx11) };

	let s00 = if current > d00 { shadow.strength } else { 1.0 };
	let s10 = if current > d10 { shadow.strength } else { 1.0 };
	let s01 = if current > d01 { shadow.strength } else { 1.0 };
	let s11 = if current > d11 { shadow.strength } else { 1.0 };

	let sx0 = s00 + (s10 - s00) * fx;
	let sx1 = s01 + (s11 - s01) * fx;
	sx0 + (sx1 - sx0) * fy
}

pub struct Flat;

impl VS for Flat {
	fn shade_vertex<'d>(
		&self,
		input: VertexIn,
		object: pscene::object::ObjectRef<'d>,
		uniforms: &super::uniform::GlobalUniforms,
	) -> VertexOut {
		let m_model = object.m_model;
		let m_normal = object.m_normal;

		let w_pos =
			(m_model * Vector4::from((input.attributes.position, 1.0))).xyz();
		let w_normal = (m_normal * Vector4::from((input.face_normal, 0.0))).xyz();
		let w_tangent =
			(m_normal * Vector4::from((input.attributes.tangent, 0.0))).xyz();
		let w_bitangent =
			(m_normal * Vector4::from((input.attributes.bi_tangent, 0.0))).xyz();

		let m_mvp = uniforms.m_projection * uniforms.m_view * m_model;

		VertexOut {
			clip: m_mvp * Vector4::from((input.attributes.position, 1.0)),
			vary: Varyings {
				uv: input.attributes.uv,
				normal: w_normal,
				tangent: w_tangent,
				bi_tangent: w_bitangent,
				world_pos: w_pos,
				intensity: 0.0,
			},
		}
	}

	fn perspective_divide(
		&self,
		input: Varyings,
		raster_in: &crate::raster::RasterIn,
	) -> Varyings {
		Varyings {
			normal: input.normal,
			uv: input.uv * raster_in.inv_w,
			tangent: input.tangent * raster_in.inv_w,
			bi_tangent: input.bi_tangent * raster_in.inv_w,
			world_pos: input.world_pos * raster_in.inv_w,
			intensity: input.intensity * raster_in.inv_w,
		}
	}
}

impl FS for Flat {
	fn shade_pixel<'d>(
		&self,
		input: Varyings,
		object: pscene::object::ObjectRef<'d>,
		uniforms: &super::uniform::GlobalUniforms,
	) -> Color {
		let material = object.model.material;

		let ng = input.normal.normalize();

		let u = input.uv.x;
		let v = input.uv.y;

		let np_world = if let Some(n_map) = material.normal {
			let t = input.tangent.normalize();
			let b = input.bi_tangent.normalize();

			// T = normalize(T - N * dot(T, N))
			// B = cross(N, T)
			let t = (t - ng * t.dot(&ng)).normalize();
			let b = (b - ng * b.dot(&ng)).normalize();

			let tbn = Matrix3::from_tbn(t, b, ng);

			let lod = uniforms.lods.normal.unwrap_or(0.0);
			// N perpatuated world normal
			let n_world = tbn * n_map.bi_sample(u, v, lod);

			n_world.normalize()
		} else {
			// Geometric normal transformed into world in vertex stage
			ng
		};

		// Albedo base color
		let color = if let Some(albedo) = material.albedo {
			let lod = uniforms.lods.albedo.unwrap_or(0.0);
			albedo.bi_sample(u, v, lod)
		} else {
			material.diffuse
		};

		let light_dir = uniforms.light.direction;

		let ambient = color * material.ambient * uniforms.light.ambient;

		let i_ng = ng.dot(&light_dir).max(0.0);
		let i_np = np_world.dot(&light_dir).max(0.0);
		let diffuse_factor = i_ng * i_np;

		let shadow = shadow_factor(&input, uniforms, np_world);
		let diff = color * uniforms.light.color * diffuse_factor * shadow;

		ambient + diff
	}

	fn perspective_interpolate(
		&self,
		input: [Varyings; 3],
		bary: (f32, f32, f32),
		inv_depth: f32,
	) -> Varyings {
		let uvs = (input[0].uv, input[1].uv, input[2].uv);
		let w_pos = (input[0].world_pos, input[1].world_pos, input[2].world_pos);
		let ints = (input[0].intensity, input[1].intensity, input[2].intensity);

		Varyings {
			uv: math::perspective_interpolate(bary, inv_depth, uvs),
			normal: input[0].normal,
			tangent: input[0].tangent,
			bi_tangent: input[0].bi_tangent,
			world_pos: math::perspective_interpolate(bary, inv_depth, w_pos),
			intensity: math::perspective_interpolate(bary, inv_depth, ints),
		}
	}

	fn sample_gradients(
		&self,
		g_varyings: &super::GVaryings,
		dx: f32,
		dy: f32,
	) -> Varyings {
		Varyings {
			uv: g_varyings.uv.sample_at(dx, dy),
			world_pos: g_varyings.world_pos.sample_at(dx, dy),
			normal: g_varyings.normal.a,
			tangent: g_varyings.tangent.a,
			bi_tangent: g_varyings.bi_tangent.a,
			intensity: 0.0,
		}
	}

	fn step_horizontal(&self, g_varyings: &GVaryings, varyings: &mut Varyings) {
		g_varyings.uv.step_x(&mut varyings.uv);
		g_varyings.world_pos.step_x(&mut varyings.world_pos);
	}

	fn step_vertical(&self, g_varyings: &GVaryings, varyings: &mut Varyings) {
		g_varyings.uv.step_y(&mut varyings.uv);
		g_varyings.world_pos.step_y(&mut varyings.world_pos);
	}

	fn recover_value(&self, varyings: &Varyings, inv_w: f32) -> Varyings {
		Varyings {
			uv: varyings.uv * inv_w,
			world_pos: varyings.world_pos * inv_w,
			normal: varyings.normal,
			tangent: varyings.tangent,
			bi_tangent: varyings.bi_tangent,
			intensity: varyings.intensity * inv_w,
		}
	}
}

pub struct BlinnPhong;

impl VS for BlinnPhong {
	fn perspective_divide(
		&self,
		input: Varyings,
		raster_in: &crate::raster::RasterIn,
	) -> Varyings {
		input * raster_in.inv_w
	}

	fn shade_vertex<'d>(
		&self,
		input: VertexIn,
		object: pscene::object::ObjectRef<'d>,
		uniforms: &super::uniform::GlobalUniforms,
	) -> VertexOut {
		let m_model = object.m_model;
		let m_normal = object.m_normal;

		let world_pos = m_model * Vector4::from((input.attributes.position, 1.0));
		let normal = m_normal * Vector4::from((input.attributes.normal, 0.0));
		let tangent = m_normal * Vector4::from((input.attributes.tangent, 0.0));
		let bi_tangent =
			m_normal * Vector4::from((input.attributes.bi_tangent, 0.0));

		let m_vp = uniforms.m_projection * uniforms.m_view;

		VertexOut {
			clip: m_vp * world_pos,
			vary: Varyings {
				uv: input.attributes.uv,
				normal: normal.xyz(),
				tangent: tangent.xyz(),
				bi_tangent: bi_tangent.xyz(),
				world_pos: world_pos.xyz(),
				intensity: 0.0,
			},
		}
	}
}

impl FS for BlinnPhong {
	fn shade_pixel<'d>(
		&self,
		input: Varyings,
		object: pscene::object::ObjectRef<'d>,
		uniforms: &super::uniform::GlobalUniforms,
	) -> Color {
		let material = object.model.material;

		let ng = input.normal.normalize();

		let u = input.uv.x;
		let v = input.uv.y;

		let np_world = if let Some(n_map) = material.normal {
			let t = input.tangent.normalize();
			let b = input.bi_tangent.normalize();

			// T = normalize(T - N * dot(T, N))
			// B = cross(N, T)
			let t = (t - ng * t.dot(&ng)).normalize();
			let b = (b - ng * b.dot(&ng)).normalize();

			let tbn = Matrix3::from_tbn(t, b, ng);

			let lod = uniforms.lods.normal.unwrap_or(0.0);

			// N (perpatuated world normal)
			let n_world = tbn * n_map.bi_sample(u, v, lod);

			n_world.normalize()
		} else {
			// N (Geometric normal already transformed into world in vertex stage)
			ng
		};

		let color = if let Some(albedo) = material.albedo {
			let lod = uniforms.lods.albedo.unwrap_or(0.0);
			albedo.tri_sample(u, v, lod)
		} else {
			material.diffuse
		};

		// L (Light didrection)
		let light_dir = uniforms.light.direction;

		// V (View direction)
		let view_dir = (uniforms.camera.position - input.world_pos).normalize();

		// H = normalize(L + V)
		let half_vec = (light_dir + view_dir).normalize();

		// Diffuse
		let shadow = shadow_factor(&input, uniforms, np_world);
		let diff = color * uniforms.light.color * np_world.dot(&light_dir).max(0.0) * shadow;

		let ambient = color * material.ambient * uniforms.light.ambient;

		// Specular factor, pow(max(dot(N, H), 0), shininess)
		// The specular factor here is calculated uisng modified
		// Schlick approximation to avoid the powf in this hot
		// pixel loop.
		let s = material.shininess;
		let ndoth = np_world.dot(&half_vec).max(0.0);
		let spec_factor = ndoth / (s - s * ndoth + ndoth);

		// Specular
		let specular = material.specular * uniforms.light.color * spec_factor * shadow;

		ambient + diff + specular
	}

	fn perspective_interpolate(
		&self,
		input: [Varyings; 3],
		bary: (f32, f32, f32),
		inv_depth: f32,
	) -> Varyings {
		math::perspective_interpolate(
			bary,
			inv_depth,
			(input[0], input[1], input[2]),
		)
	}

	fn sample_gradients(
		&self,
		g_varyings: &GVaryings,
		dx: f32,
		dy: f32,
	) -> Varyings {
		g_varyings.sample_all(dx, dy)
	}

	fn recover_value(&self, varyings: &Varyings, inv_w: f32) -> Varyings {
		*varyings * inv_w
	}

	fn step_horizontal(&self, g_varyings: &GVaryings, varyings: &mut Varyings) {
		g_varyings.step_horizontal_all(varyings);
	}

	fn step_vertical(&self, g_varyings: &GVaryings, varyings: &mut Varyings) {
		g_varyings.step_vertical_all(varyings);
	}
}

pub struct Shadows;

impl VS for Shadows {
	fn shade_vertex<'d>(
		&self,
		input: VertexIn,
		object: pscene::object::ObjectRef<'d>,
		uniforms: &super::uniform::GlobalUniforms,
	) -> VertexOut {
		let m_model = object.m_model;

		let world_pos = m_model * Vector4::from((input.attributes.position, 1.0));
		let m_vp = uniforms.m_projection * uniforms.m_view;

		VertexOut {
			clip: m_vp * world_pos,
			vary: Varyings {
				world_pos: world_pos.xyz(),
				..Default::default()
			},
		}
	}

	fn perspective_divide(
		&self,
		input: Varyings,
		_raster_in: &crate::raster::RasterIn,
	) -> Varyings {
		input
	}
}

pub struct DepthOnly;

impl VS for DepthOnly {
	fn shade_vertex<'d>(
		&self,
		input: VertexIn,
		object: pscene::object::ObjectRef<'d>,
		uniforms: &super::uniform::GlobalUniforms,
	) -> VertexOut {
		let m_model = object.m_model;
		let m_mvp = uniforms.m_projection * uniforms.m_view * m_model;

		VertexOut {
			clip: m_mvp * Vector4::from((input.attributes.position, 1.0)),
			vary: Varyings {
				world_pos: (m_model * Vector4::from((input.attributes.position, 1.0)))
					.xyz(),
				..Default::default()
			},
		}
	}

	fn perspective_divide(
		&self,
		input: Varyings,
		raster_in: &crate::raster::RasterIn,
	) -> Varyings {
		input * raster_in.inv_w
	}
}

impl FS for DepthOnly {
	fn shade_pixel<'d>(
		&self,
		_input: Varyings,
		_object: pscene::object::ObjectRef<'d>,
		_uniforms: &super::uniform::GlobalUniforms,
	) -> Color {
		Color::BLACK
	}

	fn perspective_interpolate(
		&self,
		input: [Varyings; 3],
		bary: (f32, f32, f32),
		inv_depth: f32,
	) -> Varyings {
		math::perspective_interpolate(
			bary,
			inv_depth,
			(input[0], input[1], input[2]),
		)
	}

	fn sample_gradients(
		&self,
		g_varyings: &GVaryings,
		dx: f32,
		dy: f32,
	) -> Varyings {
		g_varyings.sample_all(dx, dy)
	}

	fn recover_value(&self, varyings: &Varyings, inv_w: f32) -> Varyings {
		let mut v = *varyings;
		v.world_pos = v.world_pos * inv_w;

		v
	}

	fn step_horizontal(&self, g_varyings: &GVaryings, varyings: &mut Varyings) {
		// g_varyings.step_horizontal_all(varyings);
		g_varyings.world_pos.step_x(&mut varyings.world_pos);
	}

	fn step_vertical(&self, g_varyings: &GVaryings, varyings: &mut Varyings) {
		g_varyings.world_pos.step_y(&mut varyings.world_pos);
		// g_varyings.step_vertical_all(varyings);
	}
}
// pub struct Gouraud;

// impl VertexShader for Gouraud {
//     fn shade(&self, input: VertexIn, u:
// &GlobalUniforms) -> VertexOut {         let
// world_pos = (u.uniforms.model *
// Vector4::from((input.position, 1.0))).xyz();
// let normal = (u.uniforms.normal *
// Vector4::from((input.normal, 0.0))).xyz();
// let n = normal.normalize();         let l =
// u.light_dir.normalize();         let diff =
// n.dot(&l).max(0.0);         let intensity =
// (u.ambient + diff).min(1.0);

//         VertexOut {
//             clip: u.uniforms.mvp *
// Vector4::from((input.position, 1.0)),
//             vary: Varyings {
//                 uv: input.uv,
//                 normal: n,
//                 world_pos,
//                 intensity,
//             },
//         }
//     }
// }

// impl FragmentShader for Gouraud {
//     fn shade(&self, input: Varyings, _u:
// &GlobalUniforms, texture:
// &crate::scene::Texture) -> Color {
// texture.bi_sample(input.uv.x, input.uv.y) *
// input.intensity     } }

// pub struct Phong;

// impl VertexShader for Phong {
//     fn shade(&self, input: VertexIn, u:
// &GlobalUniforms) -> VertexOut {         let
// world_pos = (u.uniforms.model *
// Vector4::from((input.position, 1.0))).xyz();
// let normal = (u.uniforms.normal *
// Vector4::from((input.normal, 0.0))).xyz();

//         VertexOut {
//             clip: u.uniforms.mvp *
// Vector4::from((input.position, 1.0)),
//             vary: Varyings {
//                 uv: input.uv,
//                 normal,
//                 world_pos,
//                 intensity: 0.0,
//             },
//         }
//     }
// }

// impl FragmentShader for Phong {
//     fn shade(&self, input: Varyings, u:
// &GlobalUniforms, texture:
// &crate::scene::Texture) -> Color {         let
// n = input.normal.normalize();         let l =
// u.light_dir.normalize();         let v =
// (u.camera_pos - input.world_pos).normalize();
//         let h = (l + v).normalize();

//         let diff = n.dot(&l).max(0.0);
//         let spec = n
//             .dot(&h)
//             .max(0.0)
//             .powf(u.shininess)
//             * u.specular_strength;

//         let mut color =
// texture.bi_sample(input.uv.x, input.uv.y) *
// (u.ambient + diff).min(1.0);         if spec >
// 0.0 {             color = color +
// Color::new_rgb(1.0, 1.0, 1.0) * spec;         }

//         color
//     }
// }
