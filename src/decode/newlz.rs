use core::ptr;

use super::error::{
    DecodeError,
    DecodeResult,
};
use crate::bindings::root::oo2::{
    newLZ_chunk_arrays,
    newLZ_decode_chunk_phase1,
    newLZ_decode_chunk_phase2,
};
use crate::decode::literal_type::LiteralType;
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
) -> DecodeResult<usize> {
    debug_assert!(
        decompressed.len() <= BLOCK_LEN,
        "decompressed data exceeds block length"
    );

    let mut compressed_pos = 0;
    let mut decompressed_pos = 0;

    let scratch_size = compressor_scratch_memory_size(Compressor::Kraken, CHUNK_LEN);
    let mut scratch = vec![0_u8; scratch_size];

    while decompressed_pos < decompressed.len() {
        let chunk_len = (decompressed.len() - decompressed_pos).min(CHUNK_LEN);
        let chunk_pos = decompressed_pos + pos_since_reset;

        // Minimum length of a chunk is 4 bytes:
        // - 3 bytes for the header
        // - 1 byte for the payload
        if compressed.len() - compressed_pos < 4 {
            return Err(DecodeError::InvalidCompressedData(
                "Not enough data to read chunk header",
            ));
        }

        let mut chunk_compressed_len = u32::from_be_bytes([
            0,
            compressed[compressed_pos],
            compressed[compressed_pos + 1],
            compressed[compressed_pos + 2],
        ]) as usize;
        let chunk_type = LiteralType::try_from(chunk_compressed_len >> 19 & 0xF)?;
        debug_assert!(
            chunk_type == LiteralType::Sub || chunk_type == LiteralType::Raw,
            "chunk type should be either Subtract or Raw for newlz"
        );
        chunk_compressed_len &= (1 << 19) - 1;

        compressed_pos += 3;

        if chunk_compressed_len > compressed.len() - compressed_pos {
            return Err(DecodeError::InvalidCompressedData(
                "Chunk compressed length exceeds available data",
            ));
        }
        if chunk_compressed_len > chunk_len {
            return Err(DecodeError::InvalidCompressedData(
                "Chunk compressed length exceeds chunk length",
            ));
        }

        if chunk_compressed_len == chunk_len {
            // Is this the good chunk type ?
            // In oodle's source code, they check whether the chunk type is not 0,
            // with the comment `//raw`, meaning they expect the chunk type to be Raw.
            // But 0 is Sub. That's weird. Will have to check later.
            if chunk_type != LiteralType::Sub {
                return Err(DecodeError::InvalidCompressedData("Chunk type is not raw"));
            }

            decompressed[decompressed_pos..decompressed_pos + chunk_len]
                .copy_from_slice(&compressed[compressed_pos..compressed_pos + chunk_len]);
        } else {
            if chunk_len < MIN_CHUNK_LEN {
                return Err(DecodeError::InvalidCompressedData(
                    "Chunk length is too short",
                ));
            }

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
                &mut scratch,
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
    chunk_type: LiteralType,
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
            i32::from(chunk_type),
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
    chunk_type: LiteralType,
    decompressed: &mut [u8],
    chunk_pos: usize,
    chunk_arrays: &mut newLZ_chunk_arrays,
) -> usize {
    unsafe {
        let decompressed_ptr = decompressed.as_mut_ptr();

        newLZ_decode_chunk_phase2(
            i32::from(chunk_type),
            decompressed_ptr,
            decompressed.len().cast_signed(),
            chunk_pos.cast_signed(),
            chunk_arrays,
        )
        .cast_unsigned()
    }
}
