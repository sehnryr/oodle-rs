use thiserror::Error;

pub type DecodeResult<T> = core::result::Result<T, DecodeError>;

/// Error type for oodle-rs.
#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("Decompression failed")]
    DecompressionFailed,

    #[error("Decompression error: {0}")]
    DecompressionError(String),

    #[error("Invalid literal type: {0}")]
    InvalidLiteralType(usize),

    #[error("Invalid compressed data: {0}")]
    InvalidCompressedData(&'static str),
}
