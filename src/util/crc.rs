use crate::bindings::root::oo2::rrBigHash64_SIMD;

pub(crate) fn compute_crc(data: &[u8]) -> u32 {
    let crc = unsafe { rrBigHash64_SIMD(data.as_ptr() as *const _, data.len() as isize) };

    (crc & 0xFFFFFF) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_crc() {
        let data = b"Hello, world!";
        assert_eq!(compute_crc(data), 0xEF5D64);
    }
}
