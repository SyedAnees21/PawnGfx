use pcore::math::{Matrix4, Vector3};
use pscene::{camera::Camera, light::Light};

use crate::render::WinSize;

#[derive(Debug, Clone, Copy)]
pub struct GlobalUniforms {
	pub m_view: Matrix4,
	pub m_projection: Matrix4,
	pub m_view_projection: Matrix4,
	pub screen: ScreenUniforms,
	pub light: LightUniforms,
	pub camera: CameraUniforms,
}

#[derive(Debug, Clone, Copy)]
pub struct LightUniforms {
	pub position: Vector3,
	pub direction: Vector3,
	pub ambient: f64,
}

impl From<&Light> for LightUniforms {
	fn from(value: &Light) -> Self {
		Self {
			position: value.position,
			direction: value.direction(),
			ambient: value.ambient,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct CameraUniforms {
	pub position: Vector3,
	pub fov: f64,
	pub near: f64,
	pub far: f64,
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
	pub aspect: f64,
	pub width: f64,
	pub height: f64,
}

impl From<&WinSize> for ScreenUniforms {
	fn from(value: &WinSize) -> Self {
		Self {
			aspect: value.aspect(),
			width: value.width as f64,
			height: value.height as f64,
		}
	}
}
