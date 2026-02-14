use crate::{
    color::Color,
    geometry::Mesh,
    math::Vector3,
    scene::{Albedo, Texture},
};

pub struct Object {
    pub mesh: Mesh,
    pub albedo: Albedo,
    pub transform: Transform,
}

impl Object {
    pub fn new(mesh: Mesh) -> Self {
        Self {
            mesh,
            albedo: Albedo::default(),
            transform: Transform::default(),
        }
    }

    pub fn from_mesh_texture(mesh: Mesh, texture: Albedo) -> Self {
        Self {
            mesh,
            albedo: texture,
            transform: Transform::default(),
        }
    }

    #[inline(always)]
    pub fn get_transforms_props(&self) -> (Vector3, Vector3, Vector3) {
        (
            self.transform.scale,
            self.transform.position,
            self.transform.rotation,
        )
    }
}

pub struct Transform {
    pub scale: Vector3,
    pub position: Vector3,
    pub rotation: Vector3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            scale: Vector3::splat(1.0),
            position: Vector3::splat(0.0),
            rotation: Vector3::default(),
        }
    }
}
