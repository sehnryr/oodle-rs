use std::sync::LazyLock;

use oodle_rs::{compress, decompress};
use rand::Rng;
use rand::distr::Uniform;

mod oodle_sys {
    pub unsafe fn get_compressed_buffer_size_needed(decompressed_size: usize) -> usize {
        let n = unsafe {
            ::oodle_sys::OodleLZ_GetCompressedBufferSizeNeeded(
                ::oodle_sys::OodleLZ_Compressor_OodleLZ_Compressor_Kraken,
                decompressed_size as isize,
            )
        };
        n as usize
    }

    pub unsafe fn compress(decompressed: &[u8], compressed: &mut [u8]) -> usize {
        let result = unsafe {
            ::oodle_sys::OodleLZ_Compress(
                ::oodle_sys::OodleLZ_Compressor_OodleLZ_Compressor_Kraken,
                decompressed.as_ptr() as *const _,
                decompressed.len() as isize,
                compressed.as_mut_ptr() as *mut _,
                ::oodle_sys::OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_Normal,
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null_mut(),
                0,
            )
        };

        assert_ne!(result, ::oodle_sys::OODLELZ_FAILED as isize);

        result as usize
    }

    pub unsafe fn decompress(compressed: &[u8], decompressed: &mut [u8]) -> usize {
        let result = unsafe {
            ::oodle_sys::OodleLZ_Decompress(
                compressed.as_ptr() as *const _,
                compressed.len() as isize,
                decompressed.as_mut_ptr() as *mut _,
                decompressed.len() as isize,
                ::oodle_sys::OodleLZ_FuzzSafe_OodleLZ_FuzzSafe_Yes,
                ::oodle_sys::OodleLZ_CheckCRC_OodleLZ_CheckCRC_No,
                ::oodle_sys::OodleLZ_Verbosity_OodleLZ_Verbosity_None,
                std::ptr::null_mut(),
                0,
                None,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                0,
                ::oodle_sys::OodleLZ_Decode_ThreadPhase_OodleLZ_Decode_ThreadPhaseAll,
            )
        };

        assert_eq!(result, decompressed.len() as isize);

        result as usize
    }
}

static DECOMPRESSED_DATA: LazyLock<Vec<u8>> = LazyLock::new(|| {
    rand::rng()
        .sample_iter(Uniform::new_inclusive(32, 126).unwrap())
        .take(1_000_000)
        .collect::<Vec<_>>()
});
static COMPRESSED_DATA: LazyLock<Vec<u8>> = LazyLock::new(|| {
    let compressed_size =
        unsafe { oodle_sys::get_compressed_buffer_size_needed(DECOMPRESSED_DATA.len()) };

    let mut compressed = vec![0; compressed_size];
    let n = unsafe { oodle_sys::compress(&DECOMPRESSED_DATA, &mut compressed) };
    compressed.resize_with(n, || unreachable!());
    compressed
});

#[test]
fn test_compress() {
    let decompressed: &[u8] = &*DECOMPRESSED_DATA;
    let mut compressed = vec![0; COMPRESSED_DATA.len()];

    assert_eq!(
        compress(decompressed, &mut compressed).unwrap(),
        COMPRESSED_DATA.len()
    );
    assert_eq!(compressed, *COMPRESSED_DATA);
}

#[test]
fn test_decompress() {
    let compressed: &[u8] = &*COMPRESSED_DATA;
    let mut decompressed = vec![0; DECOMPRESSED_DATA.len()];

    assert_eq!(
        decompress(compressed, &mut decompressed).unwrap(),
        DECOMPRESSED_DATA.len()
    );
    assert_eq!(decompressed, *DECOMPRESSED_DATA);
}
