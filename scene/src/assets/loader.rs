use std::path::Path;

use pcore::error::PResult;

pub trait AssetLoader {
    fn load_from_file<P>(path: P) -> PResult<Self>
    where
        P: AsRef<Path>,
        Self: Sized;
}
