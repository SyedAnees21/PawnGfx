use {
	crate::{
		assets::registry::AssetRegistry,
		material::Material,
		model::{Model, ModelRef},
		texture::{Albedo, NormalMap},
	},
	pcore::{
		geometry::Mesh,
		math::{Matrix4, Vector3},
	},
};

pub struct Object {
	pub model: Model,
	pub mesh: Mesh,
	pub material: Material,
	pub albedo: Albedo,
	pub normal: NormalMap,
	pub transform: Transform,
}

impl Object {
	pub fn new(mesh: Mesh) -> Self {
		Self {
			model: Model::default(),
			mesh,
			material: Material::default(),
			albedo: Albedo::default(),
			normal: NormalMap::default(),
			transform: Transform::default(),
		}
	}

	pub fn resolve<'m>(&'m self, registry: &'m AssetRegistry) -> ObjectRef<'m> {
		let m_model = Matrix4::from_transforms(
			self.transform.position,
			self.transform.scale,
			self.transform.rotation,
		);

		let m_normal = m_model.inverse().transpose();

		ObjectRef {
			model: self.model.resolve(registry),
			m_model,
			m_normal,
		}
	}

	pub fn from_mesh_texture(mesh: Mesh, texture: Albedo) -> Self {
		Self {
			model: Model::default(),
			mesh,
			albedo: texture,
			material: Material::default(),
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

pub struct ObjectRef<'m> {
	pub model: ModelRef<'m>,
	pub m_model: Matrix4,
	pub m_normal: Matrix4,
}
