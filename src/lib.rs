//! Kraken, Mermaid, Selkie, Leviathan compression.

#![allow(unsafe_code, reason = "allow unsafe code until full implementation")]
#![deny(missing_docs)]
#![deny(
    clippy::cargo,
    clippy::complexity,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious,

    // use alloc and core instead of std
    clippy::alloc_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,

    // deny print statements
    clippy::print_stderr,
    clippy::print_stdout,

    // deny floating point arithmetic
    clippy::float_arithmetic,

    // always use expect instead of unwrap
    clippy::unwrap_used,

    clippy::allow_attributes_without_reason,
    clippy::empty_enum_variants_with_brackets,
    clippy::empty_structs_with_brackets,
    clippy::field_scoped_visibility_modifiers,
    clippy::get_unwrap,
    clippy::impl_trait_in_params,
    clippy::let_underscore_must_use,
    clippy::let_underscore_untyped,
    clippy::mem_forget,
    clippy::missing_assert_message,
    clippy::modulo_arithmetic,
    clippy::multiple_inherent_impl,
    clippy::non_zero_suggestions,
    clippy::panic,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::ref_patterns,
    clippy::renamed_function_params,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::return_and_then,
    clippy::same_name_method,
    clippy::self_named_module_files,
    clippy::semicolon_outside_block,
    clippy::single_char_lifetime_names,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_lit_chars_any,
    clippy::string_to_string,
    clippy::suspicious_xor_used_as_pow,
    clippy::tests_outside_test_module,
    clippy::try_err,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::unused_result_ok,
    clippy::unused_trait_names,
    clippy::use_debug,
    clippy::verbose_file_reads,
    clippy::wildcard_enum_match_arm,
)]

#[allow(
    clippy::all,
    clippy::complexity,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]
#[allow(unsafe_code)]
#[allow(dead_code)]
#[allow(unsafe_op_in_unsafe_fn)]
mod bindings;

mod compress;
mod decode;
mod decoder;
mod decompress;
pub mod error;
mod header;
mod model;
mod util;

pub use crate::compress::compress;
pub use crate::decompress::decompress;
pub use crate::model::*;
pub use crate::util::compression::get_compressed_buffer_size_hint;

const CHUNK_LEN: usize = 128 * 1024;
const MIN_CHUNK_LEN: usize = 128;
const BLOCK_LEN: usize = 1 << 18;
const BLOCK_HEADER_BYTES_MAX: usize = 2;
const QUANTUM_HEADER_MAX_SIZE: usize = 16;
const CHUNK_HEADER_SIZE: usize = 3;
const SCRATCH_ALIGNMENT_PAD: usize = 32;
const ARRAY_INTERNAL_MAX_SCRATCH: usize = 48 * 1024;
const MAX_SCRATCH_FOR_PHASE_HEADERS_AND_FUZZ: usize = 4096;

#[cfg(test)]
mod tests {
    use core::array::from_fn;

    use super::*;

    #[test]
    fn test_compress() {
        let decompressed = std::fs::read("test-data/raw/xml").expect("Failed to read file");

        let compressed_size_hint = get_compressed_buffer_size_hint(decompressed.len());
        let mut compressed = vec![0; compressed_size_hint];

        let result = compress(
            &decompressed,
            &mut compressed,
            Compressor::Kraken,
            CompressionLevel::Normal,
            CompressOptions::default(),
            0,
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
            fn $test_name() {
                let compressed = std::fs::read(format!(
                    "test-data/{}/{}.{}",
                    $compressor, $test, $compressor
                ))
                .unwrap();

                let (compressed, decompressed_len) = if compressed[4] == 0x8C {
                    (
                        &compressed[4..],
                        u32::from_le_bytes(from_fn(|i| compressed[i])) as usize,
                    )
                } else {
                    (
                        &compressed[8..],
                        usize::try_from(u64::from_le_bytes(from_fn(|i| compressed[i])))
                            .expect("Failed to convert u64 to usize"),
                    )
                };

                let mut decompressed = vec![0; decompressed_len];

                let result =
                    crate::decompress::decompress(&compressed, &mut decompressed, false, 0)
                        .expect("Decompression failed");

                assert!(
                    result >= decompressed_len,
                    "Decompression result is less than expected length"
                );

                let expected_decompressed =
                    std::fs::read(format!("test-data/raw/{}", $test)).unwrap();

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
                use super::*;

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
