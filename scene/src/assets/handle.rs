use std::marker::PhantomData;

use pcore::geometry::Mesh;

use crate::texture::{Albedo, NormalMap};

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