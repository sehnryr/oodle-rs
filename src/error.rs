//! This module defines the error handling types for the `oodle-rs` library.

use thiserror::Error;

/// Result type for oodle-rs.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for oodle-rs.
#[derive(Error, Debug)]
pub enum Error {
    /// Compression failed.
    #[error("Compression failed")]
    CompressionFailed,

    /// Decompression failed.
    #[error("Decompression failed")]
    DecompressionFailed,

    /// Empty buffer.
    #[error("Empty buffer: {0}")]
    EmptyBuffer(&'static str),

    /// Invalid dictionary length.
    #[error("Invalid dictionary length: {0}")]
    InvalidDictionaryLength(&'static str),

    /// Invalid dictionary base.
    #[error("Invalid dictionary base: {0}")]
    InvalidDictionaryBase(&'static str),

    /// Invalid compressor.
    #[error("Invalid compressor")]
    InvalidCompressor,

    /// Invalid input data.
    #[error("Invalid input data: {0}")]
    InvalidInput(&'static str),

    /// Invalid compressed data.
    #[error("Invalid compressed data: {0}")]
    InvalidCompressedData(&'static str),

    /// Invalid chunk size.
    #[error("Invalid chunk size")]
    InvalidChunkSize,

    /// Invalid header.
    #[error("Invalid header")]
    InvalidHeader,

    /// Unsupported chunk version.
    #[error("Unsupported chunk version: {0}")]
    UnsupportedChunkVersion(u8),
}
