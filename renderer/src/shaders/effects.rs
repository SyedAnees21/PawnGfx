use {
	crate::shaders::{
		FS, FragmentShader, GlobalUniforms, VS, Varyings, VertexIn, VertexOut,
		VertexShader,
	},
	pcore::math::{Matrix3, Vector4},
	pscene::{
		color::Color,
		texture::{Albedo, NormalMap},
	},
};

pub struct Flat;

impl VertexShader for Flat {
	fn shade(&self, input: VertexIn, u: &GlobalUniforms) -> VertexOut {
		let world_pos =
			(u.affine.model * Vector4::from((input.attributes.position, 1.0))).xyz();

		let normal =
			(u.affine.normal * Vector4::from((input.face_normal, 0.0))).xyz();
		let tangent =
			(u.affine.normal * Vector4::from((input.attributes.tangent, 0.0))).xyz();
		let bi_tangent = (u.affine.normal
			* Vector4::from((input.attributes.bi_tangent, 0.0)))
		.xyz();

		VertexOut {
			clip: u.affine.mvp * Vector4::from((input.attributes.position, 1.0)),
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

impl FragmentShader for Flat {
	fn shade(
		&self,
		input: Varyings,
		uniforms: &GlobalUniforms,
		albedo: &Albedo,
		normal: &NormalMap,
	) -> Color {
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
		let np = normal.bi_sample(u, v);

		let np_world = (tbn * np).normalize();

		let l = uniforms.light.direction;
		let i_ng = ng.dot(&l).max(0.0);
		let i_np = np_world.dot(&l).max(0.0);
		let diffuse = i_ng * i_np;

		let intensity = (uniforms.light.ambient + diffuse).min(1.0);

		albedo.bi_sample(input.uv.x, input.uv.y) * intensity
	}
}

pub struct BlinnPhong;

impl VertexShader for BlinnPhong {
	fn shade(&self, input: VertexIn, u: &GlobalUniforms) -> VertexOut {
		let world_pos =
			(u.affine.model * Vector4::from((input.attributes.position, 1.0))).xyz();

		let normal =
			(u.affine.normal * Vector4::from((input.face_normal, 0.0))).xyz();
		let tangent =
			(u.affine.normal * Vector4::from((input.attributes.tangent, 0.0))).xyz();
		let bi_tangent = (u.affine.normal
			* Vector4::from((input.attributes.bi_tangent, 0.0)))
		.xyz();

		VertexOut {
			clip: u.affine.mvp * Vector4::from((input.attributes.position, 1.0)),
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

impl FragmentShader for BlinnPhong {
	fn shade(
		&self,
		input: Varyings,
		uniforms: &GlobalUniforms,
		albedo: &Albedo,
		normal: &NormalMap,
	) -> Color {
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
		let np = normal.bi_sample(u, v);

		// N (perpatuated world normal)
		let np_world = (tbn * np).normalize();

		// L (Light didrection)
		let light_dir = uniforms.light.direction;

		// V (View direction)
		let view_dir = (uniforms.camera_pos - input.world_pos).normalize();

		// H = normalize(L + V)
		let half_vec = (light_dir + view_dir).normalize();

		// Diffuse
		let diff = np_world.dot(&light_dir).max(0.0);

		// Specular factor, max(dot(N, H), 0)
		let ndoth = np_world.dot(&half_vec).max(0.0);

		// Specular
		let specular = uniforms.specular_strength * ndoth.powf(uniforms.shininess);
		// Specular Highlight
		let highlight = Color::WHITE * specular;
		// Final intensity
		let intensity = uniforms.light.ambient + diff;

		albedo.bi_sample(input.uv.x, input.uv.y) * intensity + highlight
	}
}

impl VS for BlinnPhong {
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

		let normal = (m_normal * Vector4::from((input.face_normal, 0.0))).xyz();

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

		// Specular factor, max(dot(N, H), 0)
		let ndoth = np_world.dot(&half_vec).max(0.0);
		let spec_factor = ndoth.powf(material.shininess as f64);

		// Specular
		// let specular = uniforms.specular_strength * ndoth.powf(uniforms.shininess);
		let specular = material.specular * uniforms.light.color * spec_factor;
		// Specular Highlight
		// let highlight = Color::WHITE * specular;
		// Final intensity
		// let intensity = uniforms.light.ambient + diff;

		// object.model.material.albedo.unwrap().bi_sample(input.uv.x, input.uv.y) * intensity + highlight
		ambient + diff + specular
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
