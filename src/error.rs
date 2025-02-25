use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Compression failed")]
    CompressionFailed,

    #[error("Decompression failed")]
    DecompressionFailed,
}
