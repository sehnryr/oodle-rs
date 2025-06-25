use crate::BLOCK_HEADER_BYTES_MAX;
use crate::error::{
    Error,
    Result,
};
use crate::model::Compressor;

#[derive(Debug)]
pub struct BlockHeader {
    compressor: Compressor,
    is_memcpy: bool,
    is_reset: bool,
    has_quantum_crcs: bool,
}

impl BlockHeader {
    pub const fn compressor(&self) -> Compressor { self.compressor }

    pub const fn is_memcpy(&self) -> bool { self.is_memcpy }

    pub const fn is_reset(&self) -> bool { self.is_reset }

    pub const fn has_quantum_crcs(&self) -> bool { self.has_quantum_crcs }

    pub fn try_from_block(block: &[u8]) -> Result<(Self, usize)> {
        if block.len() < BLOCK_HEADER_BYTES_MAX {
            return Err(Error::InvalidChunkSize(block.len()));
        }

        let version = 4 + ((block[0] >> 4) & 0b11);

        if version != 4 {
            return Err(Error::UnsupportedChunkVersion(version));
        }

        let is_memcpy = (block[0] >> 6) & 0b1 == 1;
        let is_reset = (block[0] >> 7) & 0b1 == 1;

        let decode_type = block[1] & 0b0111_1111;
        let has_quantum_crcs = (block[1] >> 7) & 0b1 == 1;

        let compressor = match decode_type {
            6 => Compressor::Kraken,
            10 => Compressor::Mermaid,
            12 => Compressor::Leviathan,
            // Selkie ?
            // Hydra ?
            _ => return Err(Error::InvalidCompressor),
        };

        let header = Self {
            compressor,
            is_memcpy,
            is_reset,
            has_quantum_crcs,
        };

        Ok((header, 2))
    }
}
