use crate::{assets::registry::{AlbedoHandle, NormalHandle}, color::Color, texture::{Albedo, NormalMap}};

pub struct Material {
	pub shininess: f64,
	pub specular_strength: f64,
    pub diffuse: Color,
    pub ambient: Color,
    pub specular: Color,
	pub albedo: Option<AlbedoHandle>,
	pub normal: Option<NormalHandle>,
}

impl Default for Material {
	fn default() -> Self {
		Self {
			shininess: 64.0,
			specular_strength: 0.5,
            diffuse: Color::new_rgb_splat(0.6),
            ambient: Color::new_rgb_splat(0.1),
            specular: Color::new_rgb_splat(0.0),
			albedo: None,
			normal: None,
		}
	}
}

impl Material {
	pub const MIN_SHINE: f64 = 0.0;
	pub const MAX_SHINE: f64 = 255.0;

	pub const MAX_SPECULAR: f64 = 1.0;
	pub const MIN_SPECULAR: f64 = 0.0;

	#[inline]
	pub fn set_shininess(&mut self, shininess: f64) {
		self.shininess = shininess.clamp(Self::MIN_SHINE, Self::MAX_SHINE);
	}

	#[inline]
	pub fn set_specular(&mut self, specular: f64) {
		self.specular_strength =
			specular.clamp(Self::MIN_SPECULAR, Self::MAX_SPECULAR);
	}

	// #[inline]
	// pub fn set_albedo(&mut self, albedo: Albedo) {
	// 	self.albedo = Some(albedo);
	// }

	// #[inline]
	// pub fn set_normal_map(&mut self, normal_map: NormalMap) {
	// 	self.normal = Some(normal_map);
	// }
}
