// use crate::{
//     color::Color,
//     math::{Matrix4, AffineMatrices, Vector2, Vector3, Vector4},
//     raster,
//     shaders::GlobalUniforms,
// };

// pub type ScreenVertices = [Vector2; 3];
// pub type DepthVertices = [f64; 3];
// pub type Intensity = f64;

// pub struct Flat {
//     light_dir: Vector3,
//     normal_m: Matrix4,
// }

// impl crate::shaders::Vertex for Flat {
//     type Uniforms = GlobalUniforms;
//     type Out = Option<(ScreenVertices, DepthVertices, Intensity)>;

//     fn process_vertices(
//         &self,
//         v0: Vector3,
//         v1: Vector3,
//         v2: Vector3,
//         uniforms: Self::Uniforms,
//     ) -> Self::Out {
//         let model_m = uniforms.uniforms.model;
//         let view_m = uniforms.uniforms.view;
//         let projection_m = uniforms.uniforms.projection;
//         let normal_m = self.normal_m;

//         let mvp = projection_m * view_m * model_m;

//         let v0_clip = mvp * Vector4::from((v0, 1.0));
//         let v1_clip = mvp * Vector4::from((v1, 1.0));
//         let v2_clip = mvp * Vector4::from((v2, 1.0));

//         if v0_clip.w <= 0.0 || v1_clip.w <= 0.0 || v2_clip.w <= 0.0 {
//             return None;
//         }

//         let inv_w0 = 1.0 / v0_clip.w;
//         let inv_w1 = 1.0 / v1_clip.w;
//         let inv_w2 = 1.0 / v2_clip.w;

//         let v0_ndc = v0_clip * inv_w0;
//         let v1_ndc = v1_clip * inv_w1;
//         let v2_ndc = v2_clip * inv_w2;

//         let (s_v0, z0) =
//             raster::clip_to_screen(&v0_ndc, uniforms.screen_width, uniforms.screen_height);
//         let (s_v1, z1) =
//             raster::clip_to_screen(&v1_ndc, uniforms.screen_width, uniforms.screen_height);
//         let (s_v2, z2) =
//             raster::clip_to_screen(&v2_ndc, uniforms.screen_width, uniforms.screen_height);

//         if raster::is_backfacing(s_v0, s_v1, s_v2) {
//             return None;
//         }

//         let face_normal = ((v1 - v0).cross(&(v2 - v0))).normalize();
//         let rotated_normal = normal_m * Vector4::from((face_normal, 0.0));
//         let intensity = rotated_normal.xyz().dot(&self.light_dir).max(0.0);

//         Some(([s_v0, s_v1, s_v2], [z0, z1, z2], intensity))
//     }
// }

// impl crate::shaders::Fragment for Flat {
//     type In = (Color, Intensity);
//     type Out = Color;

//     fn process_fragment(&self, input: Self::In) -> Self::Out {
//         let (color, intensity) = input;
//         color * intensity
//     }
// }
