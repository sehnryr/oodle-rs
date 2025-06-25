use core::ptr;

use crate::bindings::root::oo2::{
    newLZ_chunk_arrays,
    newLZ_decode_chunk_phase1,
    newLZ_decode_chunk_phase2,
};
use crate::error::{
    Error,
    Result,
};
use crate::model::Compressor;
use crate::util::compression::compressor_scratch_memory_size;
use crate::{
    BLOCK_LEN,
    CHUNK_LEN,
    MIN_CHUNK_LEN,
};

pub fn decode_one(
    compressed: &[u8],
    decompressed: &mut [u8],
    pos_since_reset: usize,
    scratch: &mut [u8],
) -> Result<usize> {
    debug_assert!(
        decompressed.len() <= BLOCK_LEN,
        "decompressed data exceeds block length"
    );

    let mut compressed_pos = 0;
    let mut decompressed_pos = 0;

    while decompressed_pos < decompressed.len() {
        let chunk_len = (decompressed.len() - decompressed_pos).min(CHUNK_LEN);
        let chunk_pos = decompressed_pos + pos_since_reset;

        // Minimum length of a chunk is 4 bytes:
        // - 3 bytes for the header
        // - 1 byte for the payload
        if compressed.len() - compressed_pos < 4 {
            return Err(Error::InvalidCompressedData(
                "Not enough data to read chunk header",
            ));
        }

        let mut chunk_compressed_len = u32::from_be_bytes([
            0,
            compressed[compressed_pos],
            compressed[compressed_pos + 1],
            compressed[compressed_pos + 2],
        ]) as usize;
        let chunk_type @ (0 | 1) = chunk_compressed_len >> 19 & 0xF else {
            return Err(Error::InvalidCompressedData("Invalid chunk type"));
        };
        chunk_compressed_len &= (1 << 19) - 1;

        compressed_pos += 3;

        if chunk_compressed_len > compressed.len() - compressed_pos {
            return Err(Error::InvalidCompressedData(
                "Chunk compressed length exceeds available data",
            ));
        }
        if chunk_compressed_len > chunk_len {
            return Err(Error::InvalidCompressedData(
                "Chunk compressed length exceeds chunk length",
            ));
        }

        // Raw chunk
        if chunk_compressed_len == chunk_len {
            if chunk_type != 0 {
                return Err(Error::InvalidCompressedData("Chunk type is not raw"));
            }

            decompressed[decompressed_pos..decompressed_pos + chunk_len]
                .copy_from_slice(&compressed[compressed_pos..compressed_pos + chunk_len]);
        } else {
            if chunk_len < MIN_CHUNK_LEN {
                return Err(Error::InvalidCompressedData("Chunk length is too short"));
            }

            debug_assert!(
                scratch.len() >= compressor_scratch_memory_size(Compressor::Kraken, chunk_len),
                "scratch memory size is too small"
            );

            // This is normally created from the scratch memory,
            // but I don't want to use scratch memory for this rust reimplementation
            let mut chunk_arrays = newLZ_chunk_arrays {
                chunk_ptr: ptr::null_mut(),
                scratch_ptr: ptr::null_mut(),
                offsets: ptr::null_mut(),
                offsets_count: 0,
                excesses: ptr::null_mut(),
                excesses_count: 0,
                packets: ptr::null_mut(),
                packets_count: 0,
                literals_ptr: ptr::null_mut(),
                literals_count: 0,
            };

            decode_chunk_phase1(
                chunk_type,
                &compressed[compressed_pos..compressed_pos + chunk_compressed_len],
                &mut decompressed[decompressed_pos..decompressed_pos + chunk_len],
                chunk_pos,
                scratch,
                &mut chunk_arrays,
            );

            decode_chunk_phase2(
                chunk_type,
                &mut decompressed[decompressed_pos..decompressed_pos + chunk_len],
                chunk_pos,
                &mut chunk_arrays,
            );
        }

        compressed_pos += chunk_compressed_len;
        decompressed_pos += chunk_len;
    }

    Ok(compressed_pos)
}

fn decode_chunk_phase1(
    chunk_type: usize,
    compressed: &[u8],
    decompressed: &mut [u8],
    chunk_pos: usize,
    scratch: &mut [u8],
    chunk_arrays: &mut newLZ_chunk_arrays,
) -> usize {
    unsafe {
        let compressed_ptr = compressed.as_ptr();
        let decompressed_ptr = decompressed.as_mut_ptr();
        let scratch_ptr = scratch.as_mut_ptr();

        let result = newLZ_decode_chunk_phase1(
            i32::try_from(chunk_type).expect("invalid chunk type"),
            compressed_ptr,
            compressed_ptr.add(compressed.len()),
            decompressed_ptr,
            decompressed.len().cast_signed(),
            chunk_pos.cast_signed(),
            scratch_ptr,
            scratch_ptr.add(scratch.len()),
            chunk_arrays,
        )
        .cast_unsigned();

        debug_assert!(
            chunk_arrays.chunk_ptr == decompressed_ptr,
            "chunk_ptr mismatch"
        );
        debug_assert!(
            chunk_arrays.scratch_ptr == scratch_ptr,
            "scratch_ptr mismatch"
        );

        result
    }
}

fn decode_chunk_phase2(
    chunk_type: usize,
    decompressed: &mut [u8],
    chunk_pos: usize,
    chunk_arrays: &mut newLZ_chunk_arrays,
) -> usize {
    unsafe {
        let decompressed_ptr = decompressed.as_mut_ptr();

        newLZ_decode_chunk_phase2(
            i32::try_from(chunk_type).expect("invalid chunk type"),
            decompressed_ptr,
            decompressed.len().cast_signed(),
            chunk_pos.cast_signed(),
            chunk_arrays,
        )
        .cast_unsigned()
    }
}
