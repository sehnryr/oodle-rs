use super::error::{
    DecodeError,
    DecodeResult,
};
use crate::CHUNK_LEN;
use crate::bindings::root::oo2::{
    Leviathan_DecodeOneQuantum,
    OodleLZ_Decode_ThreadPhase,
};
use crate::model::Compressor;
use crate::util::compression::compressor_scratch_memory_size;

pub fn decode_one(
    compressed: &[u8],
    decompressed: &mut [u8],
    pos_since_reset: usize,
) -> DecodeResult<usize> {
    let decompressed_ptr = decompressed.as_mut_ptr();

    let scratch_size = compressor_scratch_memory_size(Compressor::Leviathan, CHUNK_LEN);
    let mut scratch = vec![0_u8; scratch_size];

    let read_bytes = unsafe {
        Leviathan_DecodeOneQuantum(
            decompressed_ptr,
            decompressed_ptr.add(decompressed.len()),
            compressed.as_ptr(),
            i32::try_from(compressed.len()).expect("length overflow"),
            compressed.as_ptr().add(compressed.len()),
            pos_since_reset.cast_signed(),
            scratch.as_mut_ptr().cast(),
            scratch.len().cast_signed(),
            OodleLZ_Decode_ThreadPhase::OodleLZ_Decode_ThreadPhaseAll,
        )
    }
    .cast_unsigned() as usize;

    if read_bytes == 0 {
        return Err(DecodeError::DecompressionFailed);
    }

    if read_bytes != compressed.len() {
        return Err(DecodeError::DecompressionError(format!(
            "Decompressed data does not match header: {} != {}",
            read_bytes,
            compressed.len()
        )));
    }

    Ok(read_bytes)
}
