// use crate::{
//     input::{Controller, Keys},
//     scene::{Albedo, NormalMap},
// };
use {
	crate::texture::{Albedo, NormalMap},
	pcore::{geometry::Mesh, math::Vector3},
};

pub struct Object {
	pub mesh: Mesh,
	pub albedo: Albedo,
	pub normal: NormalMap,
	pub transform: Transform,
}

impl Object {
	pub fn new(mesh: Mesh) -> Self {
		Self {
			mesh,
			albedo: Albedo::default(),
			normal: NormalMap::default(),
			transform: Transform::default(),
		}
	}

	pub fn from_mesh_texture(mesh: Mesh, texture: Albedo) -> Self {
		Self {
			mesh,
			albedo: texture,
			normal: NormalMap::default(),
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

	pub fn set_albedo(&mut self, albedo: Albedo) {
		self.albedo = albedo;
	}

	pub fn set_normal_map(&mut self, normal: NormalMap) {
		self.normal = normal;
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

// impl Controller for Object {
//     fn apply_inputs(&mut self, controller: &crate::input::InputState) {
//         if controller.is_pressed(Keys::Up) {
//             self.transform.rotation.x -= 0.9;
//         }

//         if controller.is_pressed(Keys::Down) {
//             self.transform.rotation.x += 0.9;
//         }

//         if controller.is_pressed(Keys::Left) {
//             self.transform.rotation.y -= 0.9;
//         }

//         if controller.is_pressed(Keys::Right) {
//             self.transform.rotation.y += 0.9;
//         }
//     }
// }
