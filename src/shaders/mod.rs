mod effects;
mod vertex;

pub use effects::*;
pub use vertex::*;

use crate::{
    geometry::Mesh,
    math::{Matrix4, AffineMatrices, Vector3},
};

#[derive(Debug, Clone, Copy)]
pub struct GlobalUniforms {
    pub uniforms: AffineMatrices,
    pub screen_width: f64,
    pub screen_height: f64,
}

impl HasUniforms for GlobalUniforms {
    fn model_matrix(&self) -> Matrix4 {
        self.uniforms.model
    }

    fn projection_matrix(&self) -> Matrix4 {
        self.uniforms.projection
    }

    fn view_matrix(&self) -> Matrix4 {
        self.uniforms.view
    }
}

pub trait HasUniforms {
    fn model_matrix(&self) -> Matrix4;
    fn view_matrix(&self) -> Matrix4;
    fn projection_matrix(&self) -> Matrix4;
}

pub trait Vertex {
    type Uniforms: HasUniforms + Copy;
    type Out;

    fn process_vertices(
        &self,
        v0: Vector3,
        v1: Vector3,
        v2: Vector3,
        uniforms: Self::Uniforms,
    ) -> Self::Out;
}

pub trait Fragment {
    type In;
    type Out;
    fn process_fragment(&self, input: Self::In) -> Self::Out;
}
