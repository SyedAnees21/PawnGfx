use {
	crate::texture::{Albedo, NormalMap},
	pcore::geometry::Mesh,
	std::{marker::PhantomData, sync::Arc},
};

pub struct AssetHandle<T> {
	index: u32,
	generation: u32,
	_marker: PhantomData<T>,
}

impl<T> AssetHandle<T> {
	pub fn new(index: u32, generation: u32) -> Self {
		Self {
			index,
			generation,
			_marker: PhantomData,
		}
	}
}

pub type MeshHandle = AssetHandle<Mesh>;
pub type AlbedoHandle = AssetHandle<Albedo>;
pub type NormalHandle = AssetHandle<NormalMap>;

pub struct AssetStore<T> {
	list: Vec<(u32, Arc<T>)>,
}

impl<T> Default for AssetStore<T> {
	fn default() -> Self {
		Self { list: Vec::new() }
	}
}

impl<T> AssetStore<T> {
	pub fn insert(&mut self, asset: T) -> AssetHandle<T> {
		let index = self.list.len() as u32;
		let generation = 0;
		self.list.push((generation, Arc::new(asset)));
		AssetHandle::new(index, generation)
	}

	pub fn get(&self, handle: AssetHandle<T>) -> Option<Arc<T>> {
		let (generation, asset) = self.list.get(handle.index as usize)?;
		if *generation != handle.generation {
			return None;
		}
		Some(asset.clone())
	}
}

pub struct AssetRegistry {
	meshes: AssetStore<Mesh>,
	albedos: AssetStore<Albedo>,
	normals: AssetStore<NormalMap>,
}

impl Default for AssetRegistry {
	fn default() -> Self {
		Self {
			meshes: AssetStore::default(),
			albedos: AssetStore::default(),
			normals: AssetStore::default(),
		}
	}
}

impl AssetRegistry {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert_mesh(&mut self, mesh: Mesh) -> MeshHandle {
		self.meshes.insert(mesh)
	}

	pub fn insert_albedo(&mut self, albedo: Albedo) -> AlbedoHandle {
		self.albedos.insert(albedo)
	}

	pub fn insert_normal(&mut self, normal: NormalMap) -> NormalHandle {
		self.normals.insert(normal)
	}

	pub fn get_mesh(&self, handle: MeshHandle) -> Option<Arc<Mesh>> {
		self.meshes.get(handle)
	}

	pub fn get_albedo(&self, handle: AlbedoHandle) -> Option<Arc<Albedo>> {
		self.albedos.get(handle)
	}

	pub fn get_normal(&self, handle: NormalHandle) -> Option<Arc<NormalMap>> {
		self.normals.get(handle)
	}
}

// pub struct AssetRegistry {
//     meshes: AssetStore<Mesh>,
//     albedo: AssetStore<Albedo>,
//     normal: AssetStore<NormalMap>,
// }
