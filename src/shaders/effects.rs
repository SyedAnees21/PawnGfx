use crate::{
    color::Color,
    geometry::{self, UV},
    math::Vector4,
    shaders::{FragmentShader, GlobalUniforms, Varyings, VertexIn, VertexOut, VertexShader},
};

pub struct Flat;

impl VertexShader for Flat {
    fn shade(&self, input: VertexIn, u: &GlobalUniforms) -> VertexOut {
        let world_pos = (u.affine.model * Vector4::from((input.attributes.position, 1.0))).xyz();
        let normal = (u.affine.normal * Vector4::from((input.face_normal, 0.0))).xyz();

        VertexOut {
            clip: u.affine.mvp * Vector4::from((input.attributes.position, 1.0)),
            vary: Varyings {
                uv: input.attributes.uv,
                normal,
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
        u: &GlobalUniforms,
        texture: &crate::scene::Texture,
        lod: f64,
    ) -> Color {
        let n = input.normal.normalize();
        let l = u.light.direction;
        let diff = n.dot(&l).max(0.0);
        let intensity = (u.light.ambient + diff).min(1.0);

        texture.tri_sample(input.uv.x, input.uv.y, lod) * intensity
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
