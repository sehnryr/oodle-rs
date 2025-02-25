use crate::{BLOCK_HEADER_BYTES_MAX, BLOCK_LEN, CHUNK_HEADER_SIZE, QUANTUM_HEADER_MAX_SIZE};

/// Get the compressed buffer size hint.
///
/// # Arguments
/// * `decompressed_len` - The length of the decompressed data.
///
/// # Returns
/// The minimum size of the compressed buffer to be allocated.
///
/// Note: hint size is likely to be larger than the actual compressed size.
pub fn get_compressed_buffer_size_hint(decompressed_len: usize) -> usize {
    let mut padding_per_seek_chunk = BLOCK_HEADER_BYTES_MAX + QUANTUM_HEADER_MAX_SIZE;
    padding_per_seek_chunk += CHUNK_HEADER_SIZE * 2;

    let num_seek_chunks = (decompressed_len + BLOCK_LEN - 1) / BLOCK_LEN;

    decompressed_len + num_seek_chunks * padding_per_seek_chunk
}
