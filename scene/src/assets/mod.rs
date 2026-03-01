use {
	pcore::{
		error::{FileError, PResult},
		geometry::Mesh,
	},
	std::path::Path,
};

pub mod loader;
pub mod obj;
// pub mod handle;
pub mod registry;

pub fn load_mesh_file<P>(path: P) -> PResult<Mesh>
where
	P: AsRef<Path>,
{
	let path = path.as_ref();

	let Some(extension) = path.extension() else {
		return Err(FileError::Invalid.into());
	};

	match extension.to_str().unwrap() {
		"obj" => obj::load_obj(path),
		"gltf" => unimplemented!("Will be implemented in future"),
		_ => Err(
			FileError::WrongFile("Wrong extension, expected .obj".to_string()).into(),
		),
	}
}
