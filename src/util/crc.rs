use crate::bindings::root::oo2::LZQuantumHeader_ComputeCRC;

pub(crate) fn compute_crc(data: &[u8]) -> u32 {
    unsafe { LZQuantumHeader_ComputeCRC(data.as_ptr(), data.len() as isize) }
}
