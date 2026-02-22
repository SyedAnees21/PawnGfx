// use crate::shaders::LightUniforms;
use pcore::math::Vector3;

pub struct Light {
    pub position: Vector3,
    pub ambient: f64,
}

impl Default for Light {
    fn default() -> Self {
        Self {
            position: Vector3::new(1.0, 1.0, 2.0),
            ambient: 0.1,
        }
    }
}

impl Light {
    pub fn direction(&self) -> Vector3 {
        self.position.normalize()
    }

    // pub fn uniforms(&self) -> LightUniforms {
    //     LightUniforms {
    //         position: self.position,
    //         direction: self.direction(),
    //         ambient: self.ambient,
    //     }
    // }
}
