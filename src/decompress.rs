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
/// # Panics
///
/// Panics if `dictionary_base` is not contiguous with `decompressed`.
pub fn decompress(
    compressed: &[u8],
    decompressed: &mut [u8],
    check_crc: Option<bool>,
    mut dictionary_base: Option<&mut [u8]>,
) -> Result<usize> {
    let compressed_len = compressed.len();
    let decompressed_len = decompressed.len();

    let check_crc = check_crc.unwrap_or(false);

    // Ensure dictionary_base is contiguous with decompressed
    // This is mandatory since we call functions from the Oodle library
    // TODO: Remove this check when we reimplement the Oodle library in Rust
    if let Some(ref mut dict) = dictionary_base {
        assert!(dict.as_mut_ptr() as usize + dict.len() == decompressed.as_mut_ptr() as usize);
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
            match dictionary_base {
                Some(ref mut dict) => dict.as_mut_ptr() as *mut _,
                None => std::ptr::null_mut(),
            },
            match dictionary_base {
                Some(ref mut dict) => dict.len() as isize,
                None => 0,
            } + decompressed_len as isize,
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
