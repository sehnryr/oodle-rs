use core::ptr;

use crate::bindings::root::oo2::{
    OodleLZ_CompressOptions,
    OodleLZ_CompressionLevel,
    OodleLZ_Compressor,
    OodleLZ_Jobify,
    OodleLZ_Profile,
};
use crate::error::{
    Error,
    Result,
};
use crate::model::{
    CompressOptions,
    CompressionLevel,
    Compressor,
    Profile,
};

impl TryFrom<OodleLZ_Compressor> for Compressor {
    type Error = Error;

    fn try_from(compressor: OodleLZ_Compressor) -> Result<Self> {
        match compressor {
            OodleLZ_Compressor::OodleLZ_Compressor_Kraken => Ok(Self::Kraken),
            OodleLZ_Compressor::OodleLZ_Compressor_Leviathan => Ok(Self::Leviathan),
            OodleLZ_Compressor::OodleLZ_Compressor_Mermaid => Ok(Self::Mermaid),
            OodleLZ_Compressor::OodleLZ_Compressor_Selkie => Ok(Self::Selkie),
            OodleLZ_Compressor::OodleLZ_Compressor_Hydra => Ok(Self::Hydra),
            OodleLZ_Compressor::OodleLZ_Compressor_Invalid
            | OodleLZ_Compressor::OodleLZ_Compressor_None
            | OodleLZ_Compressor::OodleLZ_Compressor_BitKnit
            | OodleLZ_Compressor::OodleLZ_Compressor_LZB16
            | OodleLZ_Compressor::OodleLZ_Compressor_LZNA
            | OodleLZ_Compressor::OodleLZ_Compressor_LZH
            | OodleLZ_Compressor::OodleLZ_Compressor_LZHLW
            | OodleLZ_Compressor::OodleLZ_Compressor_LZNIB
            | OodleLZ_Compressor::OodleLZ_Compressor_LZBLW
            | OodleLZ_Compressor::OodleLZ_Compressor_LZA
            | OodleLZ_Compressor::OodleLZ_Compressor_Count
            | OodleLZ_Compressor::OodleLZ_Compressor_Force32 => Err(Error::InvalidCompressor),
        }
    }
}

impl From<Compressor> for OodleLZ_Compressor {
    fn from(value: Compressor) -> Self {
        match value {
            Compressor::Kraken => Self::OodleLZ_Compressor_Kraken,
            Compressor::Leviathan => Self::OodleLZ_Compressor_Leviathan,
            Compressor::Mermaid => Self::OodleLZ_Compressor_Mermaid,
            Compressor::Selkie => Self::OodleLZ_Compressor_Selkie,
            Compressor::Hydra => Self::OodleLZ_Compressor_Hydra,
        }
    }
}

impl From<CompressionLevel> for OodleLZ_CompressionLevel {
    fn from(value: CompressionLevel) -> Self {
        match value {
            CompressionLevel::SuperFast => Self::OodleLZ_CompressionLevel_SuperFast,
            CompressionLevel::VeryFast => Self::OodleLZ_CompressionLevel_VeryFast,
            CompressionLevel::Fast => Self::OodleLZ_CompressionLevel_Fast,
            CompressionLevel::Normal => Self::OodleLZ_CompressionLevel_Normal,
            CompressionLevel::Optimal1 => Self::OodleLZ_CompressionLevel_Optimal1,
            CompressionLevel::Optimal2 => Self::OodleLZ_CompressionLevel_Optimal2,
            CompressionLevel::Optimal3 => Self::OodleLZ_CompressionLevel_Optimal3,
            CompressionLevel::Optimal4 => Self::OodleLZ_CompressionLevel_Optimal4,
            CompressionLevel::Optimal5 => Self::OodleLZ_CompressionLevel_Optimal5,
            CompressionLevel::HyperFast1 => Self::OodleLZ_CompressionLevel_HyperFast1,
            CompressionLevel::HyperFast2 => Self::OodleLZ_CompressionLevel_HyperFast2,
            CompressionLevel::HyperFast3 => Self::OodleLZ_CompressionLevel_HyperFast3,
            CompressionLevel::HyperFast4 => Self::OodleLZ_CompressionLevel_HyperFast4,
        }
    }
}

impl From<Profile> for OodleLZ_Profile {
    fn from(value: Profile) -> Self {
        match value {
            Profile::Main => Self::OodleLZ_Profile_Main,
            Profile::Reduced => Self::OodleLZ_Profile_Reduced,
        }
    }
}

impl From<CompressOptions> for OodleLZ_CompressOptions {
    fn from(value: CompressOptions) -> Self {
        Self {
            unused_was_verbosity: 0,
            minMatchLen: value.min_match_len(),
            seekChunkReset: i32::from(value.seek_chunk_reset()),
            seekChunkLen: value.seek_chunk_len(),
            profile: value.profile().into(),
            dictionarySize: value.dictionary_size(),
            spaceSpeedTradeoffBytes: value.space_speed_tradeoff_bytes(),
            unused_was_maxHuffmansPerChunk: 0,
            sendQuantumCRCs: i32::from(value.send_quantum_crcs()),
            maxLocalDictionarySize: value.max_local_dictionary_size(),
            makeLongRangeMatcher: i32::from(value.make_long_range_matcher()),
            matchTableSizeLog2: value.match_table_size_log2(),
            jobify: OodleLZ_Jobify::OodleLZ_Jobify_Disable,
            jobifyUserPtr: ptr::null_mut(),
            farMatchMinLen: value.far_match_min_len(),
            farMatchOffsetLog2: value.far_match_offset_log2(),
            reserved: [0; 4],
        }
    }
}
