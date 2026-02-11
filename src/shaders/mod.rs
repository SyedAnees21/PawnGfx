mod effects;

pub use effects::*;

use crate::{
    color::Color,
    math::{AffineMatrices, Vector2, Vector3, Vector4},
    scene::Texture,
};

#[derive(Debug, Clone, Copy)]
pub struct GlobalUniforms {
    pub uniforms: AffineMatrices,
    pub screen_width: f64,
    pub screen_height: f64,
    pub light_dir: Vector3,
    pub camera_pos: Vector3,
    pub ambient: f64,
    pub specular_strength: f64,
    pub shininess: f64,
}

pub struct LightUniforms {
    pub position: Vector3,
    pub direction: Vector3,
    pub ambient: f64,
}

pub struct ScreenUniforms {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct VertexIn {
    pub position: Vector3,
    pub normal: Vector3,
    pub uv: Vector2,
    pub face_normal: Vector3,
}

#[derive(Debug, Clone, Copy)]
pub struct Varyings {
    pub uv: Vector2,
    pub normal: Vector3,
    pub world_pos: Vector3,
    pub intensity: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct VertexOut {
    pub clip: Vector4,
    pub vary: Varyings,
}

pub trait VertexShader {
    fn shade(&self, input: VertexIn, u: &GlobalUniforms) -> VertexOut;
}

pub trait FragmentShader {
    fn shade(&self, input: Varyings, u: &GlobalUniforms, texture: &Texture) -> Color;
}
