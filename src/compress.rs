use crate::bindings::root::oo2::*;
use crate::error::{Error, Result};
use crate::model::{CompressOptions, CompressionLevel, Compressor};

/// Compress some data from memory to memory synchronously.
///
/// # Arguments
///
/// * `decompressed` - The data to be compressed.
/// * `compressed` - The buffer to store the compressed data.
/// * `compressor` - The compressor to use.
/// * `level` - The compression level to use.
/// * `compress_options` - The compression options to use. See [`CompressOptions`] for more details.
/// * `dictionary_base` - The base dictionary to use for decompression.
///
/// # Returns
///
/// The size of the compressed data.
///
/// # Panics
///
/// Panics if `dictionary_base` is not contiguous with `decompressed`.
pub fn compress(
    decompressed: &[u8],
    compressed: &mut [u8],
    compressor: Compressor,
    level: CompressionLevel,
    compress_options: Option<CompressOptions>,
    dictionary_base: Option<&[u8]>,
) -> Result<usize> {
    let decompressed_len = decompressed.len();

    // Ensure dictionary_base is contiguous with decompressed
    // This is mandatory since we call functions from the Oodle library
    // TODO: Remove this check when we reimplement the Oodle library in Rust
    if let Some(dict) = dictionary_base {
        assert!(dict.as_ptr() as usize + dict.len() == decompressed.as_ptr() as usize);
    }

    let n = unsafe {
        OodleLZ_Compress(
            compressor.into(),
            decompressed.as_ptr() as *const _,
            decompressed_len as isize,
            compressed.as_mut_ptr() as *mut _,
            level.into(),
            match compress_options {
                Some(options) => &options.into(),
                None => std::ptr::null(),
            },
            match dictionary_base {
                Some(dict) => dict.as_ptr() as *const _,
                None => std::ptr::null(),
            },
            std::ptr::null(),
            std::ptr::null_mut(),
            0,
        )
    };

    // If oo2_OodleLZ_Compress returns 0 (OODLELZ_FAILED)
    // it means it detected corruption
    if n == 0 {
        return Err(Error::CompressionFailed);
    } else if n < 0 {
        // The result from `oo2_OodleLZ_Compress` is non-negative.
        unreachable!()
    }

    Ok(n as usize)
}
