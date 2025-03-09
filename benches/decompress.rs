use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};

macro_rules! array_range {
    ($bytes:expr, $start:expr; .. $end:expr) => {
        ::core::array::from_fn::<_, { $end - $start }, _>(|i| $bytes[$start + i])
    };
}

fn rust_decompress(compressed: &[u8], decompressed: &mut [u8]) {
    oodle::decompress(compressed, decompressed, None, None).unwrap();
}

fn ffi_decompress(compressed: &[u8], decompressed: &mut [u8]) {
    unsafe {
        oodle_sys::OodleLZ_Decompress(
            compressed.as_ptr() as *const _,
            compressed.len() as isize,
            decompressed.as_mut_ptr() as *mut _,
            decompressed.len() as isize,
            oodle_sys::OodleLZ_FuzzSafe_OodleLZ_FuzzSafe_Yes,
            oodle_sys::OodleLZ_CheckCRC_OodleLZ_CheckCRC_No,
            oodle_sys::OodleLZ_Verbosity_OodleLZ_Verbosity_None,
            std::ptr::null_mut(),
            0,
            None,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
            oodle_sys::OodleLZ_Decode_ThreadPhase_OodleLZ_Decode_ThreadPhaseAll,
        );
    }
}

fn benchmark(c: &mut Criterion) {
    let compressed = std::fs::read("test-data/kraken/xml.kraken").unwrap();

    let (compressed, decompressed_len) = if compressed[4] == 0x8C {
        (
            &compressed[4..],
            u32::from_le_bytes(array_range!(compressed, 0; .. 4)) as usize,
        )
    } else {
        (
            &compressed[8..],
            u64::from_le_bytes(array_range!(compressed, 0; .. 8)) as usize,
        )
    };

    let mut decompressed = vec![0; decompressed_len];

    let mut group = c.benchmark_group("oodle");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("rust", |b| {
        b.iter(|| rust_decompress(compressed, &mut decompressed))
    });
    group.bench_function("ffi", |b| {
        b.iter(|| ffi_decompress(compressed, &mut decompressed))
    });

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
