use {
	crate::render::WinSize,
	pcore::{
		color::Color,
		math::{Matrix4, Vector3},
	},
	pscene::{
		camera::Camera,
		// color::Color,
		light::Light,
	},
};

#[derive(Clone, Copy)]
pub struct GlobalUniforms {
	pub m_view: Matrix4,
	pub m_projection: Matrix4,
	pub m_view_projection: Matrix4,
	pub screen: ScreenUniforms,
	pub light: LightUniforms,
	pub camera: CameraUniforms,
	pub lods: LOD,
}

#[derive(Debug, Clone, Copy)]
pub struct LOD {
	pub albedo: Option<f32>,
	pub normal: Option<f32>,
}

impl Default for LOD {
	fn default() -> Self {
		Self {
			albedo: None,
			normal: None,
		}
	}
}

#[derive(Clone, Copy)]
pub struct LightUniforms {
	pub position: Vector3,
	pub direction: Vector3,
	pub color: Color,
	pub ambient: f32,
}

impl From<&Light> for LightUniforms {
	fn from(value: &Light) -> Self {
		Self {
			position: value.position,
			direction: value.direction(),
			ambient: value.ambient,
			color: value.color,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct CameraUniforms {
	pub position: Vector3,
	pub fov: f32,
	pub near: f32,
	pub far: f32,
}

impl From<&Camera> for CameraUniforms {
	fn from(value: &Camera) -> Self {
		Self {
			position: value.position,
			fov: value.fov,
			near: value.near,
			far: value.far,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct ScreenUniforms {
	pub aspect: f32,
	pub width: f32,
	pub height: f32,
}

impl From<&WinSize> for ScreenUniforms {
	fn from(value: &WinSize) -> Self {
		Self {
			aspect: value.aspect(),
			width: value.width as f32,
			height: value.height as f32,
		}
	}
}
