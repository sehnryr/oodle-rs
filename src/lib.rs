#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(improper_ctypes)]
mod bindings;

mod error;

use crate::error::{Error, Result};

/// Compression algorithm.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compressor {
    /// Fast decompression and high compression ratios, amazing!
    #[default]
    Kraken,
    /// Leviathan = Kraken's big brother with higher compression, slightly slower decompression.
    Leviathan,
    /// Mermaid is between Kraken & Selkie - crazy fast, still decent compression.
    Mermaid,
    /// Selkie is a super-fast relative of Mermaid.  For maximum decode speed.
    Selkie,
    /// Hydra, the many-headed beast = Leviathan, Kraken, Mermaid, or Selkie
    Hydra,
}

/// Compression encoder complexity.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionLevel {
    /// Super fast mode, lower compression ratio
    SuperFast,
    /// Fastest LZ mode with still decent compression ratio
    VeryFast,
    /// Fast - good for daily use
    Fast,
    /// Standard medium speed LZ mode
    #[default]
    Normal,

    /// Optimal parse level 1 (faster optimal encoder)
    Optimal1,
    /// Optimal parse level 2 (recommended baseline optimal encoder)
    Optimal2,
    /// Optimal parse level 3 (slower optimal encoder)
    Optimal3,
    /// Optimal parse level 4 (very slow optimal encoder)
    Optimal4,
    /// Optimal parse level 5 (don't care about encode speed, maximum compression)
    Optimal5,

    /// Faster than SuperFast, less compression
    HyperFast1,
    /// Faster than HyperFast1, less compression
    HyperFast2,
    /// Faster than HyperFast2, less compression
    HyperFast3,
    /// Fastest, less compression
    HyperFast4,
}

impl CompressionLevel {
    pub const HYPER_FAST: CompressionLevel = CompressionLevel::HyperFast1;
    pub const OPTIMAL: CompressionLevel = CompressionLevel::Optimal2;
    pub const MAX: CompressionLevel = CompressionLevel::Optimal5;
    pub const MIN: CompressionLevel = CompressionLevel::HyperFast4;
}

/// Decode profile to target.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Profile {
    /// Main profile (all current features allowed)
    #[default]
    Main,
    /// Reduced profile (Kraken only, limited feature set)
    Reduced,
}

/// Controls the amount of internal threading.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Jobify {
    /// Use compressor default for level of internal job usage
    #[default]
    Default,
    /// Try to balance parallelism with increased memory usage
    Normal,
    /// Maximize parallelism even when doing so requires large amounts of memory
    Aggressive,
}

/// Options for the compressor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CompressOptions {
    /// Minimum match length.
    /// Cannot be used to reduce a compressor's default MML, but can be higher.
    /// On some types of data, a large MML (6 or 8) is a space-speed win.
    min_match_len: i32,
    /// Whether chunks should be independent, for seeking and parallelism
    seek_chunk_reset: bool,
    /// Length of independent seek chunks (if seek_chunk_reset).
    /// Must be a power of 2 and >= 1<<18
    seek_chunk_len: i32,
    /// Decoder profile to target.
    profile: Profile,
    /// Sets a maximum offset for matches, if lower than the maximum the format supports.
    /// <= 0 means infinite (use whole buffer). Often power of 2 but doesn't have to be.
    dictionary_size: i32,
    /// This is a number of bytes.
    /// I must gain at least this many bytes of compressed size to accept
    /// a speed-decreasing decision.
    space_speed_tradeoff_bytes: i32,
    /// Should the encoder send a CRC of each compressed quantum, for integrity checks.
    /// This is necessary if you want to use `check_crc` on decode.
    send_quantum_crcs: bool,
    /// Size of local dictionary before needing a long range matcher.
    /// This does not set a window size for the decoder.
    /// It's useful to limit memory use and time taken in the encoder.
    /// `max_local_dictionary_size` must be a power of 2. Must be <= 1<<30.
    max_local_dictionary_size: i32,
    /// Should the encoder find matches beyond `max_local_dictionary_size` using an LRM
    /// (long range matcher)?
    make_long_range_matcher: bool,
    /// When variable, sets the size of the match finder structure (often a hash table).
    /// Use 0 for the compressor's default.
    match_table_size_log2: i32,
    /// Controls internal job usage by compressors.
    jobify: Option<Jobify>,
    /// Far matches must be at least this len.
    far_match_min_len: i32,
    /// If not zero, the log2 of an offset that must meet `far_match_min_len`.
    far_match_offset_log2: i32,
}

impl Default for CompressOptions {
    fn default() -> Self {
        Self {
            min_match_len: 0,
            seek_chunk_reset: false,
            seek_chunk_len: 1 << 18,
            profile: Profile::default(),
            dictionary_size: 0,
            space_speed_tradeoff_bytes: 256,
            send_quantum_crcs: false,
            max_local_dictionary_size: 2 << 20,
            make_long_range_matcher: true,
            match_table_size_log2: 0,
            jobify: Some(Jobify::default()),
            far_match_min_len: 0,
            far_match_offset_log2: 0,
        }
    }
}

impl CompressOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn min_match_len(mut self, min_match_len: i32) -> Self {
        self.min_match_len = min_match_len;
        self
    }

    pub fn seek_chunk_reset(mut self, seek_chunk_reset: bool) -> Self {
        self.seek_chunk_reset = seek_chunk_reset;
        self
    }

    pub fn seek_chunk_len(mut self, seek_chunk_len: i32) -> Self {
        self.seek_chunk_len = seek_chunk_len;
        self
    }

    pub fn profile(mut self, profile: Profile) -> Self {
        self.profile = profile;
        self
    }

    pub fn dictionary_size(mut self, dictionary_size: i32) -> Self {
        self.dictionary_size = dictionary_size;
        self
    }

    pub fn space_speed_tradeoff_bytes(mut self, space_speed_tradeoff_bytes: i32) -> Self {
        self.space_speed_tradeoff_bytes = space_speed_tradeoff_bytes;
        self
    }

    pub fn send_quantum_crcs(mut self, send_quantum_crcs: bool) -> Self {
        self.send_quantum_crcs = send_quantum_crcs;
        self
    }

    pub fn max_local_dictionary_size(mut self, max_local_dictionary_size: i32) -> Self {
        self.max_local_dictionary_size = max_local_dictionary_size;
        self
    }

    pub fn make_long_range_matcher(mut self, make_long_range_matcher: bool) -> Self {
        self.make_long_range_matcher = make_long_range_matcher;
        self
    }

    pub fn match_table_size_log2(mut self, match_table_size_log2: i32) -> Self {
        self.match_table_size_log2 = match_table_size_log2;
        self
    }

    pub fn jobify(mut self, jobify: Option<Jobify>) -> Self {
        self.jobify = jobify;
        self
    }

    pub fn far_match_min_len(mut self, far_match_min_len: i32) -> Self {
        self.far_match_min_len = far_match_min_len;
        self
    }

    pub fn far_match_offset_log2(mut self, far_match_offset_log2: i32) -> Self {
        self.far_match_offset_log2 = far_match_offset_log2;
        self
    }
}

pub fn get_compressed_buffer_size_hint(decompressed_len: usize, compressor: Compressor) -> usize {
    let n = unsafe {
        bindings::oo2_OodleLZ_GetCompressedBufferSizeNeeded(
            compressor.into(),
            decompressed_len as isize,
        )
    };

    if n < 0 {
        // The result of `oo2_OodleLZ_GetCompressedBufferSizeNeeded` is non-negative.
        unreachable!()
    }

    n as usize
}

/// Compress some data from memory to memory synchronously.
///
/// # Arguments
///
/// * `decompressed` - The data to be compressed.
/// * `compressed` - The buffer to store the compressed data.
/// * `compressor` - The compressor to use.
/// * `level` - The compression level to use.
/// * `compress_options` - The compression options to use. See [`CompressOptions`] for more details.
/// * `dictionary_base` - The base dictionary to use for decompression.
///
/// # Returns
///
/// The size of the compressed data.
///
/// # Panics
///
/// Panics if `dictionary_base` is not contiguous with `decompressed`.
pub fn compress(
    decompressed: &[u8],
    compressed: &mut [u8],
    compressor: Compressor,
    level: CompressionLevel,
    compress_options: Option<CompressOptions>,
    dictionary_base: Option<&[u8]>,
) -> Result<usize> {
    let decompressed_len = decompressed.len();

    // Ensure dictionary_base is contiguous with decompressed
    // This is mandatory since we call functions from the Oodle library
    // TODO: Remove this check when we reimplement the Oodle library in Rust
    if let Some(dict) = dictionary_base {
        assert!(dict.as_ptr() as usize + dict.len() == decompressed.as_ptr() as usize);
    }

    let n = unsafe {
        bindings::oo2_OodleLZ_Compress(
            compressor.into(),
            decompressed.as_ptr() as *const _,
            decompressed_len as isize,
            compressed.as_mut_ptr() as *mut _,
            level.into(),
            match compress_options {
                Some(options) => &options.into(),
                None => std::ptr::null(),
            },
            match dictionary_base {
                Some(dict) => dict.as_ptr() as *const _,
                None => std::ptr::null(),
            },
            std::ptr::null(),
            std::ptr::null_mut(),
            0,
        )
    };

    // If oo2_OodleLZ_Compress returns 0 (OODLELZ_FAILED)
    // it means it detected corruption
    if n == 0 {
        return Err(Error::CompressionFailed);
    } else if n < 0 {
        // The result from `oo2_OodleLZ_Compress` is non-negative.
        unreachable!()
    }

    Ok(n as usize)
}

/// Decompress some data from memory to memory synchronously.
///
/// # Arguments
///
/// * `compressed` - The data to be decompressed.
/// * `decompressed` - The buffer to write the decompressed data.
/// * `check_crc` - Whether to check the CRC of the decompressed data.
/// * `dictionary_base` - The base dictionary to use for decompression.
///
/// # Returns
///
/// The number of bytes written to the decompressed buffer.
///
/// # Notes
///
/// `decompressed` must be the actual size of the decompressed data.
///
/// By default, `check_crc` is disabled and corruption is not checked.
/// If enabled, the decode will abort if corruption is detected.
///
/// # Panics
///
/// Panics if `dictionary_base` is not contiguous with `decompressed`.
pub fn decompress(
    compressed: &[u8],
    decompressed: &mut [u8],
    check_crc: Option<bool>,
    mut dictionary_base: Option<&mut [u8]>,
) -> Result<usize> {
    let compressed_len = compressed.len();
    let decompressed_len = decompressed.len();

    let check_crc = check_crc.unwrap_or(false);

    // Ensure dictionary_base is contiguous with decompressed
    // This is mandatory since we call functions from the Oodle library
    // TODO: Remove this check when we reimplement the Oodle library in Rust
    if let Some(ref mut dict) = dictionary_base {
        assert!(dict.as_mut_ptr() as usize + dict.len() == decompressed.as_mut_ptr() as usize);
    }

    let n = unsafe {
        bindings::oo2_OodleLZ_Decompress(
            compressed.as_ptr() as *const _,
            compressed_len as isize,
            decompressed.as_mut_ptr() as *mut _,
            decompressed_len as isize,
            // deprecated (always enabled)
            bindings::oo2_OodleLZ_FuzzSafe_OodleLZ_FuzzSafe_Yes,
            if check_crc {
                bindings::oo2_OodleLZ_CheckCRC_OodleLZ_CheckCRC_Yes
            } else {
                bindings::oo2_OodleLZ_CheckCRC_OodleLZ_CheckCRC_No
            },
            bindings::oo2_OodleLZ_Verbosity_OodleLZ_Verbosity_None,
            match dictionary_base {
                Some(ref mut dict) => dict.as_mut_ptr() as *mut _,
                None => std::ptr::null_mut(),
            },
            match dictionary_base {
                Some(ref mut dict) => dict.len() as isize,
                None => 0,
            } + decompressed_len as isize,
            None,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
            // always true for new lz compressors
            bindings::oo2_OodleLZ_Decode_ThreadPhase_OodleLZ_Decode_ThreadPhaseAll,
        )
    };

    // If oo2_OodleLZ_Compress returns 0 (OODLELZ_FAILED)
    // it means it detected corruption
    if n == 0 {
        return Err(Error::DecompressionFailed);
    } else if n < 0 {
        // The result from `oo2_OodleLZ_Decompress` is non-negative.
        unreachable!()
    }

    Ok(n as usize)
}

impl Into<bindings::oo2_OodleLZ_Compressor> for Compressor {
    fn into(self) -> bindings::oo2_OodleLZ_Compressor {
        match self {
            Compressor::Kraken => bindings::oo2_OodleLZ_Compressor_OodleLZ_Compressor_Kraken,
            Compressor::Leviathan => bindings::oo2_OodleLZ_Compressor_OodleLZ_Compressor_Leviathan,
            Compressor::Mermaid => bindings::oo2_OodleLZ_Compressor_OodleLZ_Compressor_Mermaid,
            Compressor::Selkie => bindings::oo2_OodleLZ_Compressor_OodleLZ_Compressor_Selkie,
            Compressor::Hydra => bindings::oo2_OodleLZ_Compressor_OodleLZ_Compressor_Hydra,
        }
    }
}

impl Into<bindings::oo2_OodleLZ_CompressionLevel> for CompressionLevel {
    fn into(self) -> bindings::oo2_OodleLZ_CompressionLevel {
        match self {
            CompressionLevel::SuperFast => {
                bindings::oo2_OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_SuperFast
            }
            CompressionLevel::VeryFast => {
                bindings::oo2_OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_VeryFast
            }
            CompressionLevel::Fast => {
                bindings::oo2_OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_Fast
            }
            CompressionLevel::Normal => {
                bindings::oo2_OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_Normal
            }
            CompressionLevel::Optimal1 => {
                bindings::oo2_OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_Optimal1
            }
            CompressionLevel::Optimal2 => {
                bindings::oo2_OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_Optimal2
            }
            CompressionLevel::Optimal3 => {
                bindings::oo2_OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_Optimal3
            }
            CompressionLevel::Optimal4 => {
                bindings::oo2_OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_Optimal4
            }
            CompressionLevel::Optimal5 => {
                bindings::oo2_OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_Optimal5
            }
            CompressionLevel::HyperFast1 => {
                bindings::oo2_OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_HyperFast1
            }
            CompressionLevel::HyperFast2 => {
                bindings::oo2_OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_HyperFast2
            }
            CompressionLevel::HyperFast3 => {
                bindings::oo2_OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_HyperFast3
            }
            CompressionLevel::HyperFast4 => {
                bindings::oo2_OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_HyperFast4
            }
        }
    }
}

impl Into<bindings::oo2_OodleLZ_Profile> for Profile {
    fn into(self) -> bindings::oo2_OodleLZ_Profile {
        match self {
            Profile::Main => bindings::oo2_OodleLZ_Profile_OodleLZ_Profile_Main,
            Profile::Reduced => bindings::oo2_OodleLZ_Profile_OodleLZ_Profile_Reduced,
        }
    }
}

impl Into<bindings::oo2_OodleLZ_Jobify> for Jobify {
    fn into(self) -> bindings::oo2_OodleLZ_Jobify {
        match self {
            Jobify::Default => bindings::oo2_OodleLZ_Jobify_OodleLZ_Jobify_Default,
            Jobify::Normal => bindings::oo2_OodleLZ_Jobify_OodleLZ_Jobify_Normal,
            Jobify::Aggressive => bindings::oo2_OodleLZ_Jobify_OodleLZ_Jobify_Aggressive,
        }
    }
}

impl Into<bindings::oo2_OodleLZ_CompressOptions> for CompressOptions {
    fn into(self) -> bindings::oo2_OodleLZ_CompressOptions {
        bindings::oo2_OodleLZ_CompressOptions {
            unused_was_verbosity: 0,
            minMatchLen: self.min_match_len,
            seekChunkReset: self.seek_chunk_reset as i32,
            seekChunkLen: self.seek_chunk_len,
            profile: self.profile.into(),
            dictionarySize: self.dictionary_size,
            spaceSpeedTradeoffBytes: self.space_speed_tradeoff_bytes,
            unused_was_maxHuffmansPerChunk: 0,
            sendQuantumCRCs: self.send_quantum_crcs as i32,
            maxLocalDictionarySize: self.max_local_dictionary_size,
            makeLongRangeMatcher: self.make_long_range_matcher as i32,
            matchTableSizeLog2: self.match_table_size_log2,
            jobify: match self.jobify {
                Some(jobify) => jobify.into(),
                None => bindings::oo2_OodleLZ_Jobify_OodleLZ_Jobify_Disable,
            },
            jobifyUserPtr: std::ptr::null_mut(),
            farMatchMinLen: self.far_match_min_len,
            farMatchOffsetLog2: self.far_match_offset_log2,
            reserved: [0; 4],
        }
    }
}

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

        let compressor = Compressor::Kraken;

        let compressed_size_hint = get_compressed_buffer_size_hint(decompressed.len(), compressor);
        let mut compressed = vec![0; compressed_size_hint];

        let result = compress(
            &decompressed,
            &mut compressed,
            compressor,
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

    #[test]
    fn test_decompress() {
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

        let result =
            decompress(&compressed, &mut decompressed, None, None).expect("Decompression failed");

        assert!(
            result >= decompressed_len,
            "Decompression result is less than expected length"
        );

        let expected_decompressed = std::fs::read("test-data/raw/xml").unwrap();

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
}
