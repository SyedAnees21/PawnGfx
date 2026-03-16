use pcore::color::Color;
use crate::{
	assets::registry::{AlbedoHandle, AssetRegistry, NormalHandle},
	// color::Color,
	texture::{AlbedoMap as Albedo, NormalMap},
};

pub struct Material {
	/// This controls the size of the specular highlight.
	/// Its the exponent on specular factor.
	pub shininess: f32,

	pub specular_strength: f32,

	/// True color of an object under direct white light.
	pub diffuse: Color,

	/// Color of an object under shadows or indirect light.
	pub ambient: Color,

	/// Color of the reflected light from the surface of an object.
	/// This gives the tint over metallic surfaces and keeps the
	/// wooden/plastic surface matte.
	pub specular: Color,

	/// A diffuse map, this gives the color texture to the object.
	/// If not present, the shader will use the diffuse color.
	pub albedo: Option<AlbedoHandle>,

	/// A normal map, enables the shader to mimic the surface details.
	/// If not  present the shader will use a flat normal.
	pub normal: Option<NormalHandle>,
}

impl Default for Material {
	fn default() -> Self {
		Self {
			shininess: 8.0,
			specular_strength: 0.5,
			diffuse: Color::from_hex_unchecked("#716f6f"),
			ambient: Color::new_rgb_splat(0.5),
			specular: Color::BLACK,
			albedo: None,
			normal: None,
		}
	}
}

impl Material {
	pub const MAX_SHINE: f32 = 255.0;
	pub const MAX_SPECULAR: f32 = 1.0;
	pub const MIN_SHINE: f32 = 0.0;
	pub const MIN_SPECULAR: f32 = 0.0;

	#[inline]
	pub fn set_shininess(&mut self, shininess: f32) {
		self.shininess = shininess.clamp(Self::MIN_SHINE, Self::MAX_SHINE);
	}

	#[inline]
	pub fn set_specular(&mut self, specular: f32) {
		self.specular_strength =
			specular.clamp(Self::MIN_SPECULAR, Self::MAX_SPECULAR);
	}

	pub fn set_albedo(&mut self, handle: AlbedoHandle) {
		self.albedo = Some(handle)
	}

	pub fn set_normal_map(&mut self, handle: NormalHandle) {
		self.normal = Some(handle)
	}

	pub fn resolve<'m>(&'m self, registry: &'m AssetRegistry) -> MaterialRef<'m> {
		MaterialRef {
			shininess: self.shininess,
			diffuse: self.diffuse,
			ambient: self.ambient,
			specular: self.specular,
			albedo: self.albedo.as_ref().and_then(|h| registry.get_albedo(h)),
			normal: self.normal.as_ref().and_then(|h| registry.get_normal(h)),
		}
	}
}

#[derive(Clone, Copy)]
pub struct MaterialRef<'m> {
	pub shininess: f32,
	pub diffuse: Color,
	pub ambient: Color,
	pub specular: Color,
	pub albedo: Option<&'m Albedo>,
	pub normal: Option<&'m NormalMap>,
}
