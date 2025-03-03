use crate::BLOCK_HEADER_BYTES_MAX;
use crate::error::{Error, Result};
use crate::model::Compressor;

pub struct BlockHeader {
    pub(crate) compressor: Compressor,
    pub(crate) is_memcpy: bool,
    pub(crate) is_reset: bool,
    pub(crate) has_quantum_crcs: bool,
}

impl BlockHeader {
    pub fn try_from_block(block: &[u8]) -> Result<Self> {
        if block.len() < BLOCK_HEADER_BYTES_MAX {
            return Err(Error::InvalidChunkSize);
        }

        let version = 4 + ((block[0] >> 4) & 0b11);

        if version != 4 {
            return Err(Error::UnsupportedChunkVersion(version));
        }

        let is_memcpy = (block[0] >> 6) & 0b1 == 1;
        let is_reset = (block[0] >> 7) & 0b1 == 1;

        let decode_type = block[1] & 0b1111111;
        let has_quantum_crcs = (block[1] >> 7) & 0b1 == 1;

        let compressor = match decode_type {
            6 => Compressor::Kraken,
            10 => Compressor::Mermaid,
            12 => Compressor::Leviathan,
            // Selkie ?
            // Hydra ?
            _ => return Err(Error::InvalidCompressor),
        };

        Ok(Self {
            compressor,
            is_memcpy,
            is_reset,
            has_quantum_crcs,
        })
    }
}
