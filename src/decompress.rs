use std::mem::size_of;
use std::{ptr, slice};

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

    let check_crc = check_crc.unwrap_or(false);

    // If dictionary_base is not provided, use decompressed as the dictionary
    let (decompressed, mut dictionary_len) = match dictionary_base {
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

            let combined = unsafe {
                slice::from_raw_parts_mut(dict.as_mut_ptr(), dict_len + decompressed.len())
            };

            (combined, dict_len)
        }
        None => (decompressed, 0),
    };

    let decode_start_offset = dictionary_len;

    let mut decompressed_pos = decode_start_offset;
    let mut compressed_pos = 0;

    if dictionary_len == 0 {
        dictionary_len = decompressed.len();
    }

    let compressor = get_all_chunks_compressor(&compressed, decompressed.len())?;

    let mut decoder = Decoder::new(compressor, decompressed.len(), decode_start_offset);

    while decompressed_pos < decompressed.len() {
        let (bytes_decoded, bytes_read) = decoder.decode_some(
            compressed,
            compressed_pos,
            decompressed,
            decompressed_pos,
            dictionary_len,
            check_crc,
        )?;

        decompressed_pos += bytes_decoded;
        compressed_pos += bytes_read;
    }

    if decompressed_pos != decompressed.len() {
        return Err(Error::DecompressionFailed);
    }

    Ok(decompressed_pos)
}

struct Decoder {
    inner: OodleLZDecoder,
    _scratch: Vec<u8>,
}

impl Decoder {
    fn new(compressor: Compressor, raw_len: usize, decode_start_offset: usize) -> Self {
        let decoder_size = size_of::<OodleLZDecoder>() + size_of::<u64>();
        let scratch_size = compressor_scratch_memory_size(compressor, raw_len);

        let memory_size = decoder_size + scratch_size;

        let mut scratch = vec![0u8; scratch_size];

        let decoder = OodleLZDecoder {
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
            scratch: scratch.as_mut_ptr() as *mut _,
            scratch_size: scratch_size as isize,
            legacy: [0u8; 64],
        };

        Self {
            inner: decoder,
            _scratch: scratch,
        }
    }

    fn decode_some(
        &mut self,
        compressed: &[u8],
        compressed_pos: usize,
        decompressed: &mut [u8],
        decompressed_pos: usize,
        dictionary_len: usize,
        check_crc: bool,
    ) -> Result<(usize, usize)> {
        if compressed.len() <= compressed_pos {
            return Err(Error::InvalidInput("compressed data is too short"));
        }

        let mut out = OodleLZ_DecodeSome_Out {
            decodedCount: 0,
            compBufUsed: 0,
            curQuantumRawLen: 0,
            curQuantumCompLen: 0,
        };

        let result = unsafe {
            OodleLZDecoder_DecodeSome(
                ptr::addr_of_mut!(self.inner) as *mut _,
                &mut out,
                decompressed.as_mut_ptr() as *mut _,
                decompressed_pos as isize,
                dictionary_len as isize,
                (dictionary_len - decompressed_pos) as isize,
                compressed.as_ptr().add(compressed_pos) as *const _,
                (compressed.len() - compressed_pos) as isize,
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

        Ok((out.decodedCount as usize, out.compBufUsed as usize))
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
