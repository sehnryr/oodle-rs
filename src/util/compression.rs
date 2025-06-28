use crate::model::Compressor;
use crate::{
    ARRAY_INTERNAL_MAX_SCRATCH,
    BLOCK_HEADER_BYTES_MAX,
    BLOCK_LEN,
    CHUNK_HEADER_SIZE,
    CHUNK_LEN,
    MAX_SCRATCH_FOR_PHASE_HEADERS_AND_FUZZ,
    QUANTUM_HEADER_MAX_SIZE,
    SCRATCH_ALIGNMENT_PAD,
};

/// Get the compressed buffer size hint.
///
/// # Arguments
/// * `decompressed_len` - The length of the decompressed data.
///
/// # Returns
/// The minimum size of the compressed buffer to be allocated.
///
/// Note: hint size is likely to be larger than the actual compressed size.
#[must_use]
pub const fn get_compressed_buffer_size_hint(decompressed_len: usize) -> usize {
    let mut padding_per_seek_chunk = BLOCK_HEADER_BYTES_MAX + QUANTUM_HEADER_MAX_SIZE;
    padding_per_seek_chunk += CHUNK_HEADER_SIZE * 2;

    let num_seek_chunks = decompressed_len.div_ceil(BLOCK_LEN);

    decompressed_len + num_seek_chunks * padding_per_seek_chunk
}

#[inline]
pub fn compressor_scratch_memory_size(
    compressor: Compressor,
    decompressed_len: usize,
) -> usize {
    let min_scratch = decompressed_len.min(CHUNK_LEN);
    let mut max_scratch = min_scratch * 2 + SCRATCH_ALIGNMENT_PAD;

    if compressor == Compressor::Kraken
        || compressor == Compressor::Leviathan
        || compressor == Compressor::Hydra
    {
        max_scratch += min_scratch;
    }

    max_scratch += ARRAY_INTERNAL_MAX_SCRATCH;
    max_scratch += MAX_SCRATCH_FOR_PHASE_HEADERS_AND_FUZZ;

    max_scratch
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compressor_scratch_memory_size() {
        assert_eq!(
            compressor_scratch_memory_size(Compressor::Kraken, CHUNK_LEN),
            446_496
        );
        assert_eq!(
            compressor_scratch_memory_size(Compressor::Leviathan, CHUNK_LEN),
            446_496
        );
        assert_eq!(
            compressor_scratch_memory_size(Compressor::Mermaid, CHUNK_LEN),
            315_424
        );
        assert_eq!(
            compressor_scratch_memory_size(Compressor::Selkie, CHUNK_LEN),
            315_424
        );
        assert_eq!(
            compressor_scratch_memory_size(Compressor::Hydra, CHUNK_LEN),
            446_496
        );
    }
}
