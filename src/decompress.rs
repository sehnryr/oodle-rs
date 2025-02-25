use const_format::concatcp;

use crate::BLOCK_LEN;
use crate::bindings;
use crate::error::{Error, Result};

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
    let (dictionary_base, dictionary_len) = match dictionary_base {
        Some(dict) => (dict.as_mut_ptr(), dict.len()),
        None => (decompressed.as_mut_ptr(), 0),
    };

    // If the dictionary length is not a multiple of BLOCK_LEN, it's an almost guaranteed failure.
    // This is a consequence of how the Oodle library handles decompress_pos. It starts at
    if dictionary_len % BLOCK_LEN != 0 {
        return Err(Error::InvalidDictionaryLength(concatcp!(
            "dictionary length must be a multiple of ",
            BLOCK_LEN
        )));
    }

    // Ensure dictionary_base is contiguous with decompressed
    // This is mandatory since we call functions from the Oodle library
    if dictionary_base as usize + dictionary_len != decompressed.as_mut_ptr() as usize {
        return Err(Error::InvalidDictionaryBase(concatcp!(
            "dictionary base must be contiguous with decompressed"
        )));
    }

    let n = unsafe {
        bindings::oo2_OodleLZ_Decompress(
            compressed.as_ptr() as *const _,
            compressed_len as isize,
            decompressed.as_mut_ptr() as *mut _,
            decompressed_len as isize,
            // deprecated (always enabled)
            bindings::oo2_OodleLZ_FuzzSafe_OodleLZ_FuzzSafe_Yes,
            if check_crc {
                bindings::oo2_OodleLZ_CheckCRC_OodleLZ_CheckCRC_Yes
            } else {
                bindings::oo2_OodleLZ_CheckCRC_OodleLZ_CheckCRC_No
            },
            bindings::oo2_OodleLZ_Verbosity_OodleLZ_Verbosity_None,
            dictionary_base as *mut _,
            dictionary_len as isize,
            None,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
            // always true for new lz compressors
            bindings::oo2_OodleLZ_Decode_ThreadPhase_OodleLZ_Decode_ThreadPhaseAll,
        )
    };

    // If oo2_OodleLZ_Compress returns 0 (OODLELZ_FAILED)
    // it means it detected corruption
    if n == 0 {
        return Err(Error::DecompressionFailed);
    } else if n < 0 {
        // The result from `oo2_OodleLZ_Decompress` is non-negative.
        unreachable!()
    }

    Ok(n as usize)
}
