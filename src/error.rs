//! This module defines the error handling types for the `oodle-rs` library.

use thiserror::Error;

/// Result type for oodle-rs.
pub type Result<T> = core::result::Result<T, Error>;

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
    #[error("Invalid chunk size: {0}")]
    InvalidChunkSize(usize),

    /// Invalid header.
    #[error("Invalid header")]
    InvalidHeader,

    /// Unsupported chunk version.
    #[error("Unsupported chunk version: {0}")]
    UnsupportedChunkVersion(u8),

    /// Invalid decompressed step.
    #[error("Invalid decompressed step: {0}")]
    InvalidDecompressedStep(usize),

    /// Invalid quantum special.
    #[error("Invalid quantum special: {0}")]
    InvalidQuantumSpecial(u32),

    /// Invalid quantum length.
    #[error("Invalid quantum length: {0}")]
    InvalidQuantumLength(usize),

    /// Invalid CRC.
    #[error("Invalid CRC: {0}")]
    InvalidCRC(String),

    /// Decompression error.
    #[error("Decompression error: {0}")]
    DecompressionError(String),

    /// Invalid chunk type.
    #[error("Invalid chunk type: {0}")]
    InvalidChunkType(usize),
}
