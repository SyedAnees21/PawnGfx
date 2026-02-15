use crate::{
    color::Color,
    math::{Matrix3, Vector4},
    scene::{Albedo, NormalMap},
    shaders::{FragmentShader, GlobalUniforms, Varyings, VertexIn, VertexOut, VertexShader},
};

pub struct Flat;

impl VertexShader for Flat {
    fn shade(&self, input: VertexIn, u: &GlobalUniforms) -> VertexOut {
        let world_pos = (u.affine.model * Vector4::from((input.attributes.position, 1.0))).xyz();

        let normal = (u.affine.normal * Vector4::from((input.face_normal, 0.0))).xyz();
        let tangent = (u.affine.normal * Vector4::from((input.attributes.tangent, 0.0))).xyz();
        let bi_tangent =
            (u.affine.normal * Vector4::from((input.attributes.bi_tangent, 0.0))).xyz();

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
        let n = input.normal.normalize();
        let t = input.tangent.normalize();
        // let b = input.bi_tangent.normalize();

        let u = input.uv.x;
        let v = input.uv.y;

        // T = normalize(T - N * dot(T, N))
        // B = cross(N, T)
        let t = (t - n * t.dot(&n)).normalize();
        // let b = n.cross(&t);
        let b = (input.bi_tangent - n * input.bi_tangent.dot(&n)).normalize();

        let tbn = Matrix3::from_tbn(t, b, n);
        let g_normal = normal.bi_sample(u, v);

        let n_world = (tbn * g_normal).normalize();

        let l = uniforms.light.direction;
        let diff = n_world.dot(&l).max(0.0);
        let intensity = (uniforms.light.ambient + diff).min(1.0);

        albedo.bi_sample(input.uv.x, input.uv.y) * intensity
    }
}

// pub struct Gouraud;

// impl VertexShader for Gouraud {
//     fn shade(&self, input: VertexIn, u: &GlobalUniforms) -> VertexOut {
//         let world_pos = (u.uniforms.model * Vector4::from((input.position, 1.0))).xyz();
//         let normal = (u.uniforms.normal * Vector4::from((input.normal, 0.0))).xyz();
//         let n = normal.normalize();
//         let l = u.light_dir.normalize();
//         let diff = n.dot(&l).max(0.0);
//         let intensity = (u.ambient + diff).min(1.0);

//         VertexOut {
//             clip: u.uniforms.mvp * Vector4::from((input.position, 1.0)),
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
//     fn shade(&self, input: Varyings, _u: &GlobalUniforms, texture: &crate::scene::Texture) -> Color {
//         texture.bi_sample(input.uv.x, input.uv.y) * input.intensity
//     }
// }

// pub struct Phong;

// impl VertexShader for Phong {
//     fn shade(&self, input: VertexIn, u: &GlobalUniforms) -> VertexOut {
//         let world_pos = (u.uniforms.model * Vector4::from((input.position, 1.0))).xyz();
//         let normal = (u.uniforms.normal * Vector4::from((input.normal, 0.0))).xyz();

//         VertexOut {
//             clip: u.uniforms.mvp * Vector4::from((input.position, 1.0)),
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
//     fn shade(&self, input: Varyings, u: &GlobalUniforms, texture: &crate::scene::Texture) -> Color {
//         let n = input.normal.normalize();
//         let l = u.light_dir.normalize();
//         let v = (u.camera_pos - input.world_pos).normalize();
//         let h = (l + v).normalize();

//         let diff = n.dot(&l).max(0.0);
//         let spec = n
//             .dot(&h)
//             .max(0.0)
//             .powf(u.shininess)
//             * u.specular_strength;

//         let mut color = texture.bi_sample(input.uv.x, input.uv.y) * (u.ambient + diff).min(1.0);
//         if spec > 0.0 {
//             color = color + Color::new_rgb(1.0, 1.0, 1.0) * spec;
//         }

//         color
//     }
// }
