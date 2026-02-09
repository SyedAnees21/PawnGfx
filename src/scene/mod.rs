mod camera;
mod object;
mod transform;

pub use camera::*;
pub use object::*;

use crate::{
    animate::ProceduralAnimator,
    draw::{CUBE_TRIS, CUBE_VERTS},
    geometry::{Mesh, Normals},
    input::InputState,
    math::Vector3,
};

pub struct Scene {
    pub camera: Camera,
    pub object: Object,
    pub light: Vector3,
    pub input: InputState,
    pub animator: ProceduralAnimator,
}

impl Default for Scene {
    fn default() -> Self {
        let camera = Camera::new(Vector3::new(0.0, 0.0, 5.0));

        let cube_mesh = Mesh::new(CUBE_VERTS.into(), CUBE_TRIS.into(), Normals::default());
        let object = Object::new(cube_mesh);

        let light = Vector3::new(1.0, 1.0, 2.0).normalize();
        let input = InputState::default();

        let animator =
            ProceduralAnimator::new(Vector3::new(15.0, 0.0, 10.0), Vector3::new(0.0, 0.0, 5.0));

        Self {
            camera,
            object,
            light,
            input,
            animator,
        }
    }
}

impl Scene {
    pub fn initialize_default() -> Self {
        Self::default()
    }
}
