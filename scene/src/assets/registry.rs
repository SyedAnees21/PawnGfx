use {
	crate::{
		material::{Material, MaterialRef},
		texture::{AlbedoMap as Albedo, NormalMap},
	},
	pcore::geometry::Mesh,
	std::marker::PhantomData,
};

#[derive(Default)]
pub struct AssetHandle<T> {
	index: u32,
	generation: u32,
	_marker: PhantomData<T>,
}

impl<T> Clone for AssetHandle<T> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<T> Copy for AssetHandle<T> {}

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
pub type MaterialHandle = AssetHandle<Material>;

pub struct AssetSlot<T> {
	generation: u32,
	asset: Option<T>,
}

pub struct AssetStore<T> {
	list: Vec<AssetSlot<T>>,
	free: Vec<u32>,
}

impl<T> Default for AssetStore<T> {
	fn default() -> Self {
		Self {
			list: Vec::new(),
			free: Vec::new(),
		}
	}
}

impl<T> AssetStore<T> {
	pub fn insert(&mut self, asset: T) -> AssetHandle<T> {
		let (index, generation) = match self.free.pop() {
			Some(index) => {
				let slot = self.list.get_mut(index as usize).unwrap();
				slot.asset = Some(asset);

				(index, slot.generation)
			}

			None => {
				let index = self.list.len() as u32;
				let generation = 0;
				self.list.push(AssetSlot {
					generation,
					asset: Some(asset),
				});

				(index, generation)
			}
		};

		AssetHandle::new(index, generation)
	}

	pub fn get(&self, handle: &AssetHandle<T>) -> Option<&T> {
		let slot = self.list.get(handle.index as usize)?;
		if slot.generation != handle.generation {
			return None;
		}
		slot.asset.as_ref()
	}

	pub fn remove(&mut self, handle: &AssetHandle<T>) -> bool {
		if let Some(slot) = self.list.get_mut(handle.index as usize)
			&& slot.generation == handle.generation
		{
			slot.generation += 1;
			slot.asset = None;
			self.free.push(handle.index);
			return true;
		}
		false
	}
}

macro_rules! impl_asset_type {
	($name:ident, $type:ty, $handle:ty) => {
		paste::paste! {
				pub fn [<insert_ $name>](&mut self, asset: $type) -> $handle {
						self.[<$name s>].insert(asset)
				}

				pub fn [<get_ $name>](&self, handle: &$handle) -> Option<&$type> {
						self.[<$name s>].get(handle)
				}

				pub fn [<remove_ $name>](&mut self, handle: &$handle) -> bool {
						self.[<$name s>].remove(handle)
				}
		}
	};
}

#[derive(Default)]
pub struct AssetRegistry {
	meshs: AssetStore<Mesh>,
	albedos: AssetStore<Albedo>,
	normals: AssetStore<NormalMap>,
	materials: AssetStore<Material>,
}

impl AssetRegistry {
	impl_asset_type!(mesh, Mesh, MeshHandle);

	impl_asset_type!(albedo, Albedo, AlbedoHandle);

	impl_asset_type!(normal, NormalMap, NormalHandle);

	impl_asset_type!(material, Material, MaterialHandle);

	pub fn new() -> Self {
		Self::default()
	}

	pub fn get_material_ref<'m>(
		&'m self,
		handle: &'m MaterialHandle,
	) -> MaterialRef<'m> {
		let mtl = self.get_material(handle).unwrap();
		mtl.resolve(self)
	}
}
