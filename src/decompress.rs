use std::slice;

use const_format::concatcp;

use crate::BLOCK_LEN;
use crate::decoder::Decoder;
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

    if dictionary_len == 0 {
        dictionary_len = decompressed.len();
    }

    let mut decoder = Decoder::new(
        compressed,
        decompressed,
        decode_start_offset,
        dictionary_len,
        check_crc,
    )?;

    decoder.decode()
}
