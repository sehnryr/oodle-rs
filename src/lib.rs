//! Kraken, Mermaid, Selkie, Leviathan compression.

#![warn(unsafe_code)]
#![deny(missing_docs)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[allow(dead_code)]
#[allow(unsafe_op_in_unsafe_fn)]
mod bindings;

mod compress;
mod decoder;
mod decompress;
pub mod error;
mod header;
mod model;
mod util;

pub use crate::model::*;

pub use crate::compress::compress;
pub use crate::decompress::decompress;
pub use crate::util::compression::get_compressed_buffer_size_hint;

const CHUNK_LEN: usize = 128 * 1024;
const BLOCK_LEN: usize = 1 << 18;
const BLOCK_HEADER_BYTES_MAX: usize = 2;
const QUANTUM_HEADER_MAX_SIZE: usize = 16;
const CHUNK_HEADER_SIZE: usize = 3;
const SCRATCH_ALIGNMENT_PAD: usize = 32;
const ARRAY_INTERNAL_MAX_SCRATCH: usize = 48 * 1024;
const MAX_SCRATCH_FOR_PHASE_HEADERS_AND_FUZZ: usize = 4096;

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! array_range {
        ($bytes:expr, $start:expr; .. $end:expr) => {
            core::array::from_fn::<_, $end, _>(|i| $bytes[$start + i])
        };
    }

    #[test]
    fn test_compress() {
        let decompressed = std::fs::read("test-data/raw/xml").unwrap();

        let compressed_size_hint = get_compressed_buffer_size_hint(decompressed.len());
        let mut compressed = vec![0; compressed_size_hint];

        let result = compress(
            &decompressed,
            &mut compressed,
            Compressor::Kraken,
            CompressionLevel::Normal,
            None,
            None,
        )
        .expect("Compression failed");

        compressed.resize_with(result, || unreachable!());

        assert!(
            compressed.len() <= compressed_size_hint,
            "Compression result is larger than expected"
        );
    }

    macro_rules! test_decompress {
        ($test_name:ident, $compressor:expr, $test:expr) => {
            #[test]
            fn $test_name () {
                let compressed = std::fs::read(format!("test-data/{}/{}.{}", $compressor, $test, $compressor)).unwrap();

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

                let result =
                crate::decompress::decompress(&compressed, &mut decompressed, None, None).expect("Decompression failed");

                assert!(
                    result >= decompressed_len,
                    "Decompression result is less than expected length"
                );

                let expected_decompressed = std::fs::read(format!("test-data/raw/{}", $test)).unwrap();

                assert_eq!(
                    expected_decompressed.len(),
                    decompressed.len(),
                    "Decompression did not match expected result",
                );

                assert_eq!(
                    expected_decompressed, decompressed,
                    "Decompression did not match expected result",
                );
            }
        };
    }

    macro_rules! test_suite_decompress {
        ($compressor:ident) => {
            mod $compressor {
                test_decompress!(test_decompress_dickens, stringify!($compressor), "dickens");
                test_decompress!(test_decompress_mozilla, stringify!($compressor), "mozilla");
                test_decompress!(test_decompress_mr, stringify!($compressor), "mr");
                test_decompress!(test_decompress_nci, stringify!($compressor), "nci");
                test_decompress!(test_decompress_ooffice, stringify!($compressor), "ooffice");
                test_decompress!(test_decompress_osdb, stringify!($compressor), "osdb");
                test_decompress!(test_decompress_reymont, stringify!($compressor), "reymont");
                test_decompress!(test_decompress_samba, stringify!($compressor), "samba");
                test_decompress!(test_decompress_sao, stringify!($compressor), "sao");
                test_decompress!(test_decompress_webster, stringify!($compressor), "webster");
                test_decompress!(test_decompress_xray, stringify!($compressor), "x-ray");
                test_decompress!(test_decompress_xml, stringify!($compressor), "xml");
            }
        };
    }

    test_suite_decompress!(kraken);
    test_suite_decompress!(leviathan);
    test_suite_decompress!(mermaid);
    test_suite_decompress!(selkie);
}
