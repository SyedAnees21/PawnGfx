use {
	image::ImageError,
	std::{
		io,
		num::{ParseFloatError, ParseIntError},
	},
	thiserror::Error,
	winit::error::{EventLoopError, OsError},
};

pub type PResult<T> = Result<T, PError>;

#[derive(Debug, Error)]
pub enum PError {
	#[error("Unable to create the window: {0}")]
	WindowCreation(#[from] OsError),

	#[error("Buffer error: {0}")]
	Buffer(#[from] BufferError),

	#[error("Event loop error: {0}")]
	EventLoop(#[from] EventLoopError),

	#[error("Error occured while asset loading: {0}")]
	Loader(#[from] FileError),

	#[error("Error occured while texture loading: {0}")]
	TextureLoading(#[from] ImageError),
}

#[derive(Debug, Error)]
pub enum BufferError {
	#[error("Unable to create frame buffer: {0}")]
	FrameBuffer(#[from] pixels::Error),

	#[error("Unable to resize frame buffer: {0}")]
	Resize(#[from] pixels::TextureError),
}

#[derive(Debug, Error)]
pub enum FileError {
	#[error("Invalid file provided")]
	Invalid,

	#[error("Wrong file: {0}")]
	WrongFile(String),

	#[error("IO error occured: {0}")]
	IOError(#[from] io::Error),

	#[error("Error occured while parsing float value: {0}")]
	Parse(#[from] ParseFloatError),

	#[error("Error occured while parsing Int value: {0}")]
	ParseInt(#[from] ParseIntError),
}

// impl From<pixels::TextureError> for PError {
// 	fn from(value: pixels::TextureError) -> Self {
// 		value.into()
// 	}
// }

// impl From<pixels::Error> for PError {
// 	fn from(value: pixels::Error) -> Self {
// 		value.into()
// 	}
// }

// impl From<io::Error> for PError {
// 	fn from(value: io::Error) -> Self {
// 		value.into()
// 	}
// }

// impl From<ParseFloatError> for PError {
// 	fn from(value: ParseFloatError) -> Self {
// 		value.into()
// 	}
// }

// impl From<ParseIntError> for PError {
// 	fn from(value: ParseIntError) -> Self {
// 		value.into()
// 	}
// }

impl From<pixels::TextureError> for PError {
	fn from(value: pixels::TextureError) -> Self {
		// Wrap the inner error in the Buffer variant,
		// which then wraps the TextureError variant.
		PError::Buffer(BufferError::Resize(value))
	}
}

impl From<pixels::Error> for PError {
	fn from(value: pixels::Error) -> Self {
		PError::Buffer(BufferError::FrameBuffer(value))
	}
}

impl From<io::Error> for PError {
	fn from(value: io::Error) -> Self {
		// FileError is wrapped in PError::Loader
		PError::Loader(FileError::IOError(value))
	}
}

impl From<ParseFloatError> for PError {
	fn from(value: ParseFloatError) -> Self {
		PError::Loader(FileError::Parse(value))
	}
}

impl From<ParseIntError> for PError {
	fn from(value: ParseIntError) -> Self {
		PError::Loader(FileError::ParseInt(value))
	}
}
