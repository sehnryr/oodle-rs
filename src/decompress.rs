use std::mem::size_of;
use std::ptr;

use const_format::concatcp;

use crate::bindings::root::oo2::*;
use crate::error::{Error, Result};
use crate::model::Compressor;
use crate::{
    ARRAY_INTERNAL_MAX_SCRATCH, BLOCK_LEN, CHUNK_LEN, MAX_SCRATCH_FOR_PHASE_HEADERS_AND_FUZZ,
    SCRATCH_ALIGNMENT_PAD,
};

/// Decompress some data from memory to memory synchronously.
///
/// # Arguments
///
/// * `compressed` - The data to be decompressed.
/// * `decompressed` - The buffer to write the decompressed data.
/// * `check_crc` - Whether to check the CRC of the decompressed data.
/// * `dictionary_base` - The base dictionary to use for decompression.
///
/// # Returns
///
/// The number of bytes written to the decompressed buffer.
///
/// # Notes
///
/// `decompressed` must be the actual size of the decompressed data.
///
/// By default, `check_crc` is disabled and corruption is not checked.
/// If enabled, the decode will abort if corruption is detected.
///
/// `dictionary_base` must be contiguous with `decompressed`.
pub fn decompress(
    compressed: &[u8],
    decompressed: &mut [u8],
    check_crc: Option<bool>,
    dictionary_base: Option<&mut [u8]>,
) -> Result<usize> {
    // If the decompressed buffer is empty, return an error
    if decompressed.len() == 0 {
        return Err(Error::EmptyBuffer("decompressed buffer is empty"));
    }

    let compressed_len = compressed.len();
    let decompressed_len = decompressed.len();

    let check_crc = check_crc.unwrap_or(false);

    // If dictionary_base is not provided, use decompressed as the dictionary
    let (dictionary_base, mut dictionary_len) = match dictionary_base {
        Some(dict) => {
            let dict_len = dict.len();

            // Ensure dictionary_base is contiguous with decompressed
            // This is mandatory since we call functions from the Oodle library
            if dict.as_ptr() as usize + dict_len != decompressed.as_mut_ptr() as usize {
                return Err(Error::InvalidDictionaryBase(concatcp!(
                    "dictionary base must be contiguous with decompressed"
                )));
            }

            // If decode_start_offset is not a multiple of BLOCK_LEN, it's an almost guaranteed failure.
            if dict_len % BLOCK_LEN != 0 {
                return Err(Error::InvalidDictionaryLength(concatcp!(
                    "dictionary length must be a multiple of ",
                    BLOCK_LEN
                )));
            }

            (dict, dict_len)
        }
        None => (decompressed, 0),
    };

    let decode_start_offset = dictionary_len;

    let mut raw_decoded = decode_start_offset;
    let raw_len = decode_start_offset + decompressed_len;
    let mut compressed_used = 0;

    if dictionary_len == 0 {
        dictionary_len = raw_len;
    }

    let compressor = get_all_chunks_compressor(&compressed, raw_len)?;

    let decoder = Decoder::new(compressor, raw_len, decode_start_offset);

    while raw_decoded < raw_len {
        if compressed_len <= compressed_used {
            panic!("compressed data is too short");
        }

        let mut out = OodleLZ_DecodeSome_Out {
            decodedCount: 0,
            compBufUsed: 0,
            curQuantumRawLen: 0,
            curQuantumCompLen: 0,
        };

        let result = unsafe {
            OodleLZDecoder_DecodeSome(
                decoder.decoder_ptr as *mut _,
                &mut out,
                dictionary_base.as_mut_ptr() as *mut _,
                raw_decoded as isize,
                dictionary_len as isize,
                (dictionary_len - raw_decoded) as isize,
                (compressed.as_ptr() as usize + compressed_used) as *const _,
                (compressed_len - compressed_used) as isize,
                OodleLZ_FuzzSafe::OodleLZ_FuzzSafe_Yes,
                if check_crc {
                    OodleLZ_CheckCRC::OodleLZ_CheckCRC_Yes
                } else {
                    OodleLZ_CheckCRC::OodleLZ_CheckCRC_No
                },
                OodleLZ_Verbosity::OodleLZ_Verbosity_None,
                OodleLZ_Decode_ThreadPhase::OodleLZ_Decode_ThreadPhaseAll,
            )
        };

        // result is bool
        if result == 0 {
            return Err(Error::DecompressionFailed);
        }

        if out.decodedCount == 0 {
            return Err(Error::DecompressionFailed);
        }

        raw_decoded += out.decodedCount as usize;
        compressed_used += out.compBufUsed as usize;
    }

    if raw_decoded != raw_len {
        return Err(Error::DecompressionFailed);
    }

    Ok(raw_decoded)
}

struct Decoder {
    _memory: Vec<u8>,
    decoder_ptr: *mut OodleLZDecoder,
}

impl Decoder {
    fn new(compressor: Compressor, raw_len: usize, decode_start_offset: usize) -> Self {
        let decoder_size = size_of::<OodleLZDecoder>() + size_of::<u64>();
        let scratch_size = compressor_scratch_memory_size(compressor, raw_len);

        let memory_size = decoder_size + scratch_size;
        let mut memory = vec![0u8; memory_size];
        let memory_ptr = memory.as_mut_ptr();

        let decoder_ptr = memory_ptr as *mut OodleLZDecoder;

        unsafe {
            ptr::write(
                decoder_ptr,
                OodleLZDecoder {
                    decPos: decode_start_offset as i64,
                    decLen: raw_len as i64,
                    gotHeaderPos: -1,
                    resetPos: 0,
                    check: 0,
                    callsWithoutProgress: 0,
                    ownsmem: true as i32,
                    header: LZBlockHeader {
                        version: 0,
                        decodeType: 0,
                        offsetShift: 0,
                        chunkIsMemcpy: 0,
                        chunkIsReset: 0,
                        chunkHasQuantumCRCs: 0,
                    },
                    decoderSize: decoder_size as i32,
                    memorySize: memory_size as i32,
                    scratch: decoder_ptr.add(1) as *mut _,
                    scratch_size: scratch_size as isize,
                    legacy: [0u8; 64],
                },
            );
        }

        Self {
            _memory: memory,
            decoder_ptr,
        }
    }
}

fn get_all_chunks_compressor(compressed: &[u8], raw_len: usize) -> Result<Compressor> {
    let compressed_len = compressed.len();

    let compressor = unsafe {
        OodleLZ_GetAllChunksCompressor(
            compressed.as_ptr() as *const _,
            compressed_len as isize,
            raw_len as isize,
        )
    };

    compressor.try_into()
}

#[inline]
fn compressor_scratch_memory_size(compressor: Compressor, raw_len: usize) -> usize {
    let min_scratch = raw_len.min(CHUNK_LEN);
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
