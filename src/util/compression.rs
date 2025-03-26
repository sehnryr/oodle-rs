use crate::error::{
    Error,
    Result,
};
use crate::header::{
    BlockHeader,
    QuantumHeader,
};
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
pub fn get_compressed_buffer_size_hint(decompressed_len: usize) -> usize {
    let mut padding_per_seek_chunk = BLOCK_HEADER_BYTES_MAX + QUANTUM_HEADER_MAX_SIZE;
    padding_per_seek_chunk += CHUNK_HEADER_SIZE * 2;

    let num_seek_chunks = (decompressed_len + BLOCK_LEN - 1) / BLOCK_LEN;

    decompressed_len + num_seek_chunks * padding_per_seek_chunk
}

fn get_chunk_compressor(compressed_chunk: &[u8]) -> Result<Compressor> {
    let (block_header, _) = BlockHeader::try_from_block(compressed_chunk)?;
    Ok(block_header.compressor)
}

pub(crate) fn get_all_chunks_compressor(
    compressed: &[u8],
    decompressed_len: usize,
) -> Result<Compressor> {
    let mut compressor = get_chunk_compressor(compressed)?;

    // > Optimize common case:
    // > Compressor can only change at BLOCK granularity
    // > so anything smaller will just have the same compressor
    if decompressed_len <= BLOCK_LEN {
        return Ok(compressor);
    }

    let mut compressed_pos = 0;
    let mut available_bytes = compressed.len();
    let mut remaining_bytes = decompressed_len;

    while available_bytes != 0 {
        let decompressed_step = remaining_bytes.min(BLOCK_LEN);
        let compressed_step = get_compressed_step_for_decompressed_step(
            &compressed[compressed_pos..],
            decompressed_step,
        )?;

        if compressed_step == 0 || compressed_step > available_bytes {
            return Err(Error::InvalidCompressedData("invalid chunk compressor"));
        }

        if compressed_step == available_bytes {
            break;
        }

        compressed_pos += compressed_step;
        available_bytes -= compressed_step;
        remaining_bytes -= decompressed_step;

        if remaining_bytes == 0 {
            break;
        }

        let current_compressor = get_chunk_compressor(&compressed[compressed_pos..])?;

        // Mix of compressor types
        if compressor != current_compressor {
            compressor = Compressor::Hydra;
        }
    }

    Ok(compressor)
}

fn get_compressed_step_for_decompressed_step(
    compressed_chunk: &[u8],
    decompressed_step: usize,
) -> Result<usize> {
    let mut compressed_pos = 0;
    let mut decompressed_pos = 0;

    while decompressed_pos < decompressed_step {
        let next_block_pos = decompressed_step.min(BLOCK_LEN);
        let chunk_size = next_block_pos - decompressed_pos;

        if decompressed_pos % BLOCK_LEN != 0 {
            return Err(Error::InvalidDecompressedStep(decompressed_step));
        }

        let (block_header, offset) =
            BlockHeader::try_from_block(&compressed_chunk[compressed_pos..])?;
        compressed_pos += offset;

        if block_header.is_memcpy {
            if compressed_chunk.len() - compressed_pos < chunk_size {
                return Ok(compressed_pos);
            }

            decompressed_pos += chunk_size;
            compressed_pos += chunk_size;
        } else {
            let (quantum_header, offset) = QuantumHeader::try_from(
                &compressed_chunk[compressed_pos..],
                block_header.has_quantum_crcs,
                chunk_size,
            )?;

            compressed_pos += offset;
            compressed_pos += quantum_header.compressed_len;
            decompressed_pos += chunk_size;
        }
    }

    Ok(compressed_pos)
}

#[inline]
pub(crate) fn compressor_scratch_memory_size(
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
            446496
        );
        assert_eq!(
            compressor_scratch_memory_size(Compressor::Leviathan, CHUNK_LEN),
            446496
        );
        assert_eq!(
            compressor_scratch_memory_size(Compressor::Mermaid, CHUNK_LEN),
            315424
        );
        assert_eq!(
            compressor_scratch_memory_size(Compressor::Selkie, CHUNK_LEN),
            315424
        );
        assert_eq!(
            compressor_scratch_memory_size(Compressor::Hydra, CHUNK_LEN),
            446496
        );
    }
}
