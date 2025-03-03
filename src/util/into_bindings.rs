use crate::bindings::root::oo2::*;
use crate::block_header::BlockHeader;
use crate::error::{Error, Result};
use crate::model::*;

impl TryFrom<OodleLZ_Compressor> for Compressor {
    type Error = Error;

    fn try_from(compressor: OodleLZ_Compressor) -> Result<Self> {
        use OodleLZ_Compressor::*;
        match compressor {
            OodleLZ_Compressor_Kraken => Ok(Compressor::Kraken),
            OodleLZ_Compressor_Leviathan => Ok(Compressor::Leviathan),
            OodleLZ_Compressor_Mermaid => Ok(Compressor::Mermaid),
            OodleLZ_Compressor_Selkie => Ok(Compressor::Selkie),
            OodleLZ_Compressor_Hydra => Ok(Compressor::Hydra),
            _ => Err(Error::InvalidCompressor),
        }
    }
}

impl Into<OodleLZ_Compressor> for Compressor {
    fn into(self) -> OodleLZ_Compressor {
        use OodleLZ_Compressor::*;
        match self {
            Compressor::Kraken => OodleLZ_Compressor_Kraken,
            Compressor::Leviathan => OodleLZ_Compressor_Leviathan,
            Compressor::Mermaid => OodleLZ_Compressor_Mermaid,
            Compressor::Selkie => OodleLZ_Compressor_Selkie,
            Compressor::Hydra => OodleLZ_Compressor_Hydra,
        }
    }
}

impl Into<OodleLZ_CompressionLevel> for CompressionLevel {
    fn into(self) -> OodleLZ_CompressionLevel {
        use OodleLZ_CompressionLevel::*;
        match self {
            CompressionLevel::SuperFast => OodleLZ_CompressionLevel_SuperFast,
            CompressionLevel::VeryFast => OodleLZ_CompressionLevel_VeryFast,
            CompressionLevel::Fast => OodleLZ_CompressionLevel_Fast,
            CompressionLevel::Normal => OodleLZ_CompressionLevel_Normal,
            CompressionLevel::Optimal1 => OodleLZ_CompressionLevel_Optimal1,
            CompressionLevel::Optimal2 => OodleLZ_CompressionLevel_Optimal2,
            CompressionLevel::Optimal3 => OodleLZ_CompressionLevel_Optimal3,
            CompressionLevel::Optimal4 => OodleLZ_CompressionLevel_Optimal4,
            CompressionLevel::Optimal5 => OodleLZ_CompressionLevel_Optimal5,
            CompressionLevel::HyperFast1 => OodleLZ_CompressionLevel_HyperFast1,
            CompressionLevel::HyperFast2 => OodleLZ_CompressionLevel_HyperFast2,
            CompressionLevel::HyperFast3 => OodleLZ_CompressionLevel_HyperFast3,
            CompressionLevel::HyperFast4 => OodleLZ_CompressionLevel_HyperFast4,
        }
    }
}

impl Into<OodleLZ_Profile> for Profile {
    fn into(self) -> OodleLZ_Profile {
        use OodleLZ_Profile::*;
        match self {
            Profile::Main => OodleLZ_Profile_Main,
            Profile::Reduced => OodleLZ_Profile_Reduced,
        }
    }
}

impl Into<OodleLZ_Jobify> for Jobify {
    fn into(self) -> OodleLZ_Jobify {
        use OodleLZ_Jobify::*;
        match self {
            Jobify::Default => OodleLZ_Jobify_Default,
            Jobify::Normal => OodleLZ_Jobify_Normal,
            Jobify::Aggressive => OodleLZ_Jobify_Aggressive,
        }
    }
}

impl Into<OodleLZ_CompressOptions> for CompressOptions {
    fn into(self) -> OodleLZ_CompressOptions {
        OodleLZ_CompressOptions {
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
                None => OodleLZ_Jobify::OodleLZ_Jobify_Disable,
            },
            jobifyUserPtr: std::ptr::null_mut(),
            farMatchMinLen: self.far_match_min_len,
            farMatchOffsetLog2: self.far_match_offset_log2,
            reserved: [0; 4],
        }
    }
}

impl Into<LZBlockHeader> for BlockHeader {
    fn into(self) -> LZBlockHeader {
        LZBlockHeader {
            version: 4,
            decodeType: match self.compressor {
                Compressor::Kraken => 6,
                Compressor::Mermaid => 10,
                Compressor::Leviathan => 12,
                _ => unreachable!(),
            },
            offsetShift: 0,
            chunkIsMemcpy: self.is_memcpy as i32,
            chunkIsReset: self.is_reset as i32,
            chunkHasQuantumCRCs: self.has_quantum_crcs as i32,
        }
    }
}
