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
}
