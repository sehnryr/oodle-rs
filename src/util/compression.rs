use crate::bindings;
use crate::model::Compressor;

/// Get the compressed buffer size hint.
///
/// # Arguments
/// * `decompressed_len` - The length of the decompressed data.
/// * `compressor` - The compressor to use.
///
/// # Returns
/// The minimum size of the compressed buffer to be allocated.
///
/// Note: hint size is likely to be larger than the actual compressed size.
pub fn get_compressed_buffer_size_hint(decompressed_len: usize, compressor: Compressor) -> usize {
    let n = unsafe {
        bindings::oo2_OodleLZ_GetCompressedBufferSizeNeeded(
            compressor.into(),
            decompressed_len as isize,
        )
    };

    if n < 0 {
        // The result of `oo2_OodleLZ_GetCompressedBufferSizeNeeded` is non-negative.
        unreachable!()
    }

    n as usize
}
