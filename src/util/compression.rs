use crate::bindings;
use crate::model::Compressor;

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
