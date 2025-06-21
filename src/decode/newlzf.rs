use crate::bindings::root::oo2::*;
use crate::error::{
    Error,
    Result,
};

pub fn decode_one(
    compressed: &[u8],
    decompressed: &mut [u8],
    pos_since_reset: usize,
    scratch: &mut [u8],
) -> Result<usize> {
    let base_decompressed_ptr = decompressed.as_mut_ptr();
    let decompressed_ptr = base_decompressed_ptr.clone();

    let read_bytes = unsafe {
        Mermaid_DecodeOneQuantum(
            decompressed_ptr,
            decompressed_ptr.add(decompressed.len()),
            compressed.as_ptr(),
            compressed.len() as i32,
            compressed.as_ptr().add(compressed.len()),
            pos_since_reset as isize,
            scratch.as_mut_ptr() as *mut _,
            scratch.len() as isize,
            OodleLZ_Decode_ThreadPhase::OodleLZ_Decode_ThreadPhaseAll,
        ) as usize
    };

    if read_bytes == 0 {
        return Err(Error::DecompressionFailed);
    }

    if read_bytes != compressed.len() {
        return Err(Error::DecompressionError(format!(
            "Decompressed data does not match header: {} != {}",
            read_bytes,
            compressed.len()
        )));
    }

    Ok(read_bytes)
}
