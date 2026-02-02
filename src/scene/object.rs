use crate::{geometry::Mesh, math::Vector3};

pub struct Object {
    pub mesh: Mesh,
    pub transform: Transform,
}

impl Object {
    pub fn new(mesh: Mesh) -> Self {
        Self {
            mesh,
            transform: Transform::default(),
        }
    }
}

#[derive(Default)]
pub struct Transform {
    pub scale: Vector3,
    pub position: Vector3,
    pub rotation: Vector3,
}
