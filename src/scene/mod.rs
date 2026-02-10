mod camera;
mod object;
mod transform;
mod texture;

pub use camera::*;
pub use object::*;
pub use texture::*;

use crate::{animate::ProceduralAnimator, input::InputState, math::Vector3};

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

        let cube_mesh = crate::loaders::load_mesh_file("./assets/meshes/cube-local.obj").unwrap();
        let texture = Texture::from_file("./assets/texture/Checker-Texture.png").unwrap();
        let object = Object::from_mesh_texture(cube_mesh, texture);

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
