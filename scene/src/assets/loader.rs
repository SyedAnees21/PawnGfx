use {pcore::error::PResult, std::path::Path};

pub trait AssetLoader {
	type Args;
	fn load_from_file<P>(path: P, args: Self::Args) -> PResult<Self>
	where
		P: AsRef<Path>,
		Self: Sized;
}
