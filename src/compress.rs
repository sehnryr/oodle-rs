use crate::bindings::root::oo2::*;
use crate::error::{
    Error,
    Result,
};
use crate::model::{
    CompressOptions,
    CompressionLevel,
    Compressor,
};

/// Compress some data from memory to memory synchronously.
///
/// # Arguments
///
/// * `decompressed` - The data to be compressed.
/// * `compressed` - The buffer to store the compressed data.
/// * `compressor` - The compressor to use.
/// * `level` - The compression level to use.
/// * `compress_options` - The compression options to use. See
///   [`CompressOptions`] for more details.
/// * `dictionary_len` - The length of the base dictionary.
///
/// # Returns
///
/// The size of the compressed data.
///
/// # Notes
///
/// A dictionary base can be provided within the `decompressed` buffer
/// at [0..dictionary_len].
pub fn compress(
    decompressed: &[u8],
    compressed: &mut [u8],
    compressor: Compressor,
    level: CompressionLevel,
    compress_options: CompressOptions,
    dictionary_len: usize,
) -> Result<usize> {
    let dictionary_base = &decompressed[..dictionary_len];
    let decompressed = &decompressed[dictionary_len..];

    let n = unsafe {
        OodleLZ_Compress(
            compressor.into(),
            decompressed.as_ptr() as *const _,
            decompressed.len() as isize,
            compressed.as_mut_ptr() as *mut _,
            level.into(),
            &compress_options.into(),
            dictionary_base.as_ptr() as *const _,
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
