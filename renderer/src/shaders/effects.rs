use {
	crate::shaders::{FS, VS, Varyings, VertexIn, VertexOut},
	pcore::math::{self, Matrix3, Vector4},
	pscene::color::Color,
};

pub struct Flat;

// impl VertexShader for Flat {
// 	fn shade(&self, input: VertexIn, u: &GlobalUniforms) -> VertexOut {
// 		let world_pos =
// 			(u.affine.model * Vector4::from((input.attributes.position, 1.0))).xyz();

// 		let normal =
// 			(u.affine.normal * Vector4::from((input.face_normal, 0.0))).xyz();
// 		let tangent =
// 			(u.affine.normal * Vector4::from((input.attributes.tangent, 0.0))).xyz();
// 		let bi_tangent = (u.affine.normal
// 			* Vector4::from((input.attributes.bi_tangent, 0.0)))
// 		.xyz();

// 		VertexOut {
// 			clip: u.affine.mvp * Vector4::from((input.attributes.position, 1.0)),
// 			vary: Varyings {
// 				uv: input.attributes.uv,
// 				normal,
// 				tangent,
// 				bi_tangent,
// 				world_pos,
// 				intensity: 0.0,
// 			},
// 		}
// 	}
// }

// impl FragmentShader for Flat {
// 	fn shade(
// 		&self,
// 		input: Varyings,
// 		uniforms: &GlobalUniforms,
// 		albedo: &Albedo,
// 		normal: &NormalMap,
// 	) -> Color {
// 		let ng = input.normal.normalize();
// 		let t = input.tangent.normalize();
// 		let b = input.bi_tangent.normalize();

// 		let u = input.uv.x;
// 		let v = input.uv.y;

// 		// T = normalize(T - N * dot(T, N))
// 		// B = cross(N, T)
// 		let t = (t - ng * t.dot(&ng)).normalize();
// 		let b = (b - ng * b.dot(&ng)).normalize();

// 		let tbn = Matrix3::from_tbn(t, b, ng);
// 		let np = normal.bi_sample(u, v);

// 		let np_world = (tbn * np).normalize();

// 		let l = uniforms.light.direction;
// 		let i_ng = ng.dot(&l).max(0.0);
// 		let i_np = np_world.dot(&l).max(0.0);
// 		let diffuse = i_ng * i_np;

// 		let intensity = (uniforms.light.ambient + diffuse).min(1.0);

// 		albedo.bi_sample(input.uv.x, input.uv.y) * intensity
// 	}
// }

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
		let t = input.tangent.normalize();
		let b = input.bi_tangent.normalize();

		let u = input.uv.x;
		let v = input.uv.y;

		// T = normalize(T - N * dot(T, N))
		// B = cross(N, T)
		let t = (t - ng * t.dot(&ng)).normalize();
		let b = (b - ng * b.dot(&ng)).normalize();

		let tbn = Matrix3::from_tbn(t, b, ng);
		let np = material.normal.unwrap().bi_sample(u, v);

		let np_world = (tbn * np).normalize();

		let color = material.albedo.unwrap().bi_sample(u, v);

		let light_dir = uniforms.light.direction.normalize();

		let ambient = color * material.ambient * uniforms.light.ambient;

		// let l = uniforms.light.direction;
		let i_ng = ng.dot(&light_dir).max(0.0);
		let i_np = np_world.dot(&light_dir).max(0.0);
		let diffuse_factor = i_ng * i_np;

		let diff = color * uniforms.light.color * diffuse_factor;

		ambient + diff
	}

	fn perspective_interpolate(
		&self,
		input: [Varyings; 3],
		bary: (f64, f64, f64),
		inv_depth: f64,
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
}

pub struct BlinnPhong;

// impl VertexShader for BlinnPhong {
// 	fn shade(&self, input: VertexIn, u: &GlobalUniforms) -> VertexOut {
// 		let world_pos =
// 			(u.affine.model * Vector4::from((input.attributes.position, 1.0))).xyz();

// 		let normal =
// 			(u.affine.normal * Vector4::from((input.face_normal, 0.0))).xyz();
// 		let tangent =
// 			(u.affine.normal * Vector4::from((input.attributes.tangent, 0.0))).xyz();
// 		let bi_tangent = (u.affine.normal
// 			* Vector4::from((input.attributes.bi_tangent, 0.0)))
// 		.xyz();

// 		VertexOut {
// 			clip: u.affine.mvp * Vector4::from((input.attributes.position, 1.0)),
// 			vary: Varyings {
// 				uv: input.attributes.uv,
// 				normal,
// 				tangent,
// 				bi_tangent,
// 				world_pos,
// 				intensity: 0.0,
// 			},
// 		}
// 	}
// }

// impl FragmentShader for BlinnPhong {
// 	fn shade(
// 		&self,
// 		input: Varyings,
// 		uniforms: &GlobalUniforms,
// 		albedo: &Albedo,
// 		normal: &NormalMap,
// 	) -> Color {
// 		let ng = input.normal.normalize();
// 		let t = input.tangent.normalize();
// 		let b = input.bi_tangent.normalize();

// 		let u = input.uv.x;
// 		let v = input.uv.y;

// 		// T = normalize(T - N * dot(T, N))
// 		// B = cross(N, T)
// 		let t = (t - ng * t.dot(&ng)).normalize();
// 		let b = (b - ng * b.dot(&ng)).normalize();

// 		let tbn = Matrix3::from_tbn(t, b, ng);
// 		let np = normal.bi_sample(u, v);

// 		// N (perpatuated world normal)
// 		let np_world = (tbn * np).normalize();

// 		// L (Light didrection)
// 		let light_dir = uniforms.light.direction;

// 		// V (View direction)
// 		let view_dir = (uniforms.camera_pos - input.world_pos).normalize();

// 		// H = normalize(L + V)
// 		let half_vec = (light_dir + view_dir).normalize();

// 		// Diffuse
// 		let diff = np_world.dot(&light_dir).max(0.0);

// 		// Specular factor, max(dot(N, H), 0)
// 		let ndoth = np_world.dot(&half_vec).max(0.0);

// 		// Specular
// 		let specular = uniforms.specular_strength * ndoth.powf(uniforms.shininess);
// 		// Specular Highlight
// 		let highlight = Color::WHITE * specular;
// 		// Final intensity
// 		let intensity = uniforms.light.ambient + diff;

// 		albedo.bi_sample(input.uv.x, input.uv.y) * intensity + highlight
// 	}
// }

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

		let world_pos =
			(m_model * Vector4::from((input.attributes.position, 1.0))).xyz();

		let normal = (m_normal * Vector4::from((input.attributes.normal, 0.0))).xyz();

		let tangent =
			(m_normal * Vector4::from((input.attributes.tangent, 0.0))).xyz();
		let bi_tangent =
			(m_normal * Vector4::from((input.attributes.bi_tangent, 0.0))).xyz();

		let m_mvp = uniforms.m_projection * uniforms.m_view * m_model;

		VertexOut {
			clip: m_mvp * Vector4::from((input.attributes.position, 1.0)),
			vary: Varyings {
				uv: input.attributes.uv,
				normal,
				tangent,
				bi_tangent,
				world_pos,
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
		let t = input.tangent.normalize();
		let b = input.bi_tangent.normalize();

		let u = input.uv.x;
		let v = input.uv.y;

		// Albedo Color
		let color = material.albedo.unwrap().bi_sample(u, v);

		// T = normalize(T - N * dot(T, N))
		// B = cross(N, T)
		let t = (t - ng * t.dot(&ng)).normalize();
		let b = (b - ng * b.dot(&ng)).normalize();

		let tbn = Matrix3::from_tbn(t, b, ng);
		let np = material.normal.unwrap().bi_sample(u, v);

		// N (perpatuated world normal)
		let np_world = (tbn * np).normalize();

		// L (Light didrection)
		let light_dir = uniforms.light.direction;

		// V (View direction)
		let view_dir = (uniforms.camera.position - input.world_pos).normalize();

		// H = normalize(L + V)
		let half_vec = (light_dir + view_dir).normalize();

		// Diffuse
		let diff = color * uniforms.light.color * np_world.dot(&light_dir).max(0.0);

		let ambient = color * material.ambient * uniforms.light.ambient;

		// Specular factor, pow(max(dot(N, H), 0), shininess)
		let ndoth = np_world.dot(&half_vec).max(0.0);
		let spec_factor = ndoth.powf(material.shininess as f64);

		// Specular
		let specular = material.specular * uniforms.light.color * spec_factor;

		ambient + diff + specular
	}

	fn perspective_interpolate(
		&self,
		input: [Varyings; 3],
		bary: (f64, f64, f64),
		inv_depth: f64,
	) -> Varyings {
		math::perspective_interpolate(
			bary,
			inv_depth,
			(input[0], input[1], input[2]),
		)
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
