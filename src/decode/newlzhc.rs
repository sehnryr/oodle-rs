use super::error::{
    DecodeError,
    DecodeResult,
};
use crate::bindings::root::oo2::{
    Leviathan_DecodeOneQuantum,
    OodleLZ_Decode_ThreadPhase,
};

pub fn decode_one(
    compressed: &[u8],
    decompressed: &mut [u8],
    pos_since_reset: usize,
    scratch: &mut [u8],
) -> DecodeResult<usize> {
    let decompressed_ptr = decompressed.as_mut_ptr();

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
