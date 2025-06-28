use const_format::concatcp;

use crate::BLOCK_LEN;
use crate::decoder::Decoder;
use crate::error::{
    Error,
    Result,
};

/// Decompress some data from memory to memory synchronously.
///
/// # Arguments
///
/// * `compressed` - The data to be decompressed.
/// * `decompressed` - The buffer to write the decompressed data.
/// * `check_crc` - Whether to check the CRC of the decompressed data.
/// * `dictionary_len` - The length of the base dictionary.
///
/// # Returns
///
/// The number of bytes written to the decompressed buffer.
///
/// # Errors
///
/// # Notes
///
/// A dictionary base can be provided within the `decompressed` buffer
/// at `[0..dictionary_len]`.
///
/// `decompressed.len() - dictionary_len` must be the actual size of the
/// decompressed data.
///
/// If `check_crc` is true, the decoder will abort if corruption is detected.
pub fn decompress(
    compressed: &[u8],
    decompressed: &mut [u8],
    check_crc: bool,
    mut dictionary_len: usize,
) -> Result<usize> {
    // If the decompressed buffer is empty, return an error
    if decompressed.is_empty() {
        return Err(Error::EmptyBuffer("decompressed buffer is empty"));
    }

    if dictionary_len % BLOCK_LEN != 0 {
        return Err(Error::InvalidDictionaryLength(concatcp!(
            "dictionary length must be a multiple of ",
            BLOCK_LEN
        )));
    }

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
    );

    decoder.decode()
}
