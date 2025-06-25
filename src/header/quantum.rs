use crate::error::{
    Error,
    Result,
};

const FLAG_SHIFT: usize = 18;
const FLAG_SPECIAL_WHOLEMATCH: u32 = 0;
const FLAG_SPECIAL_MEMSET: u32 = 1;
const FLAG_SPECIAL_MEMCPY: u32 = 2;

#[derive(Debug)]
pub struct QuantumHeader {
    compressed_len: usize,
    crc: Option<u32>,
    _whole_match: bool,
    _whole_match_offset: usize,

    huff: bool,
    extra: bool,
}

impl QuantumHeader {
    pub const fn compressed_len(&self) -> usize { self.compressed_len }

    pub const fn crc(&self) -> Option<u32> { self.crc }

    pub fn try_from(
        block: &[u8],
        has_quantum_crcs: bool,
        chunk_size: usize,
    ) -> Result<(Self, usize)> {
        if block.len() < 3 {
            return Err(Error::InvalidChunkSize(block.len()));
        }

        let mut check_crc = false;
        let mut offset = 3;
        let mut header = Self {
            compressed_len: 0,
            crc: None,
            _whole_match: false,
            _whole_match_offset: 0,
            huff: false,
            extra: false,
        };

        let raw = u32::from_be_bytes([0, block[0], block[1], block[2]]);
        let packed_compressed_len = raw & ((1 << FLAG_SHIFT) - 1);

        if packed_compressed_len == ((1 << FLAG_SHIFT) - 1) {
            let special = raw >> FLAG_SHIFT;

            if special == FLAG_SPECIAL_WHOLEMATCH {
                todo!()
            } else if special == FLAG_SPECIAL_MEMSET {
                if block.len() < 4 {
                    return Err(Error::InvalidChunkSize(block.len()));
                }

                header.crc = Some(u32::from(block[3]));
            } else if special == FLAG_SPECIAL_MEMCPY {
                header.compressed_len = chunk_size;

                check_crc = true;
            } else {
                return Err(Error::InvalidQuantumSpecial(special));
            }
        } else {
            header.compressed_len = packed_compressed_len as usize + 1;
            header.huff = raw >> FLAG_SHIFT & 0b1 == 0b1;
            header.extra = raw >> FLAG_SHIFT & 0b10 == 0b10;

            check_crc = true;
        }

        if check_crc && has_quantum_crcs {
            if block.len() < 6 {
                return Err(Error::InvalidChunkSize(block.len()));
            }

            header.crc = Some(u32::from_be_bytes([0, block[3], block[4], block[5]]));
            offset += 3;
        }

        Ok((header, offset))
    }
}
