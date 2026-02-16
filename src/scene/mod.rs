mod camera;
mod light;
mod object;
mod texture;
mod transform;

pub use camera::*;
pub use light::*;
pub use object::*;
pub use texture::*;

use crate::{
    animate::ProceduralAnimator,
    input::{Controller, InputState},
    math::Vector3,
};

pub struct Scene {
    pub camera: Camera,
    pub object: Object,
    pub light: Light,
    pub input: InputState,
    pub animator: ProceduralAnimator,
}

impl Default for Scene {
    fn default() -> Self {
        let camera = Camera::new(Vector3::new(0.0, 0.0, 5.0));

        let cube_mesh = crate::loaders::load_mesh_file("./assets/meshes/cube-local.obj").unwrap();

        let albedo = Albedo::from_file("./assets/texture/Checker-Texture.png", Wrap::Mirror).unwrap();

        let normal = NormalMap::from_file("./assets/texture/checker-normal.png", Wrap::Repeat).unwrap();

        let mut object = Object::new(cube_mesh);

        object.set_albedo(albedo);
        object.set_normal_map(normal);

        let light = Light::default();
        let input = InputState::default();

        let animator = ProceduralAnimator::new(Vector3::new(15.0, 0.0, 10.0), Vector3::new(0.0, 0.0, 5.0));

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

    pub fn update(&mut self, ism: &InputState) {
        if !self.animator.is_complete() {
            self.camera.position = self.animator.step(0.005);
            return;
        }

        self.camera.apply_inputs(ism);
        self.object.apply_inputs(ism);
    }
}
