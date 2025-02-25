use crate::bindings;
use crate::model::*;

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
