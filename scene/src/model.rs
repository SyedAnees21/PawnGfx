use {
	crate::{
		assets::registry::{AssetRegistry, MaterialHandle, MeshHandle},
		material::MaterialRef,
	},
	pcore::geometry::Mesh,
};

#[derive(Default)]
pub struct Model {
	pub mesh: MeshHandle,
	pub material: MaterialHandle,
}

impl Model {
	pub fn resolve<'m>(&'m self, registry: &'m AssetRegistry) -> ModelRef<'m> {
		ModelRef {
			mesh: registry.get_mesh(&self.mesh).unwrap(),
			material: registry.get_material_ref(&self.material),
		}
	}
}

#[derive(Clone, Copy)]
pub struct ModelRef<'m> {
	pub mesh: &'m Mesh,
	pub material: MaterialRef<'m>,
}
