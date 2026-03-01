use {
	pcore::math::{Matrix3, Vector4},
	pscene::{
		color::Color,
		texture::{Albedo, NormalMap},
	},
};

use crate::{
	// color::Color,
	// scene::{Albedo, NormalMap},
	shaders::{
		FragmentShader, GlobalUniforms, Varyings, VertexIn, VertexOut, VertexShader,
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
