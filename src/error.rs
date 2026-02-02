use thiserror::Error;
use winit::error::{EventLoopError, OsError};

pub type PResult<T> = Result<T, PError>;

#[derive(Debug, Error)]
pub enum PError {
    #[error("Unable to create the window: {0}")]
    WindowCreation(#[from] OsError),

    #[error("Buffer error: {0}")]
    Buffer(#[from] BufferError),

    #[error("Event loop error: {0}")]
    EventLoop(#[from] EventLoopError),
}

#[derive(Debug, Error)]
pub enum BufferError {
    #[error("Unable to create frame buffer: {0}")]
    FrameBuffer(#[from] pixels::Error),

    #[error("Unable to resize frame buffer: {0}")]
    Resize(#[from] pixels::TextureError),
}

impl From<pixels::TextureError> for PError {
    fn from(value: pixels::TextureError) -> Self {
        value.into()
    }
}

impl From<pixels::Error> for PError {
    fn from(value: pixels::Error) -> Self {
        value.into()
    }
}
