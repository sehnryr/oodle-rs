use crate::BLOCK_LEN;
use crate::decode::newlz::decode_one as newlz_decode_one;
use crate::decode::newlzf::decode_one as newlzf_decode_one;
use crate::decode::newlzhc::decode_one as newlzhc_decode_one;
use crate::error::{
    Error,
    Result,
};
use crate::header::{
    BlockHeader,
    QuantumHeader,
};
use crate::model::Compressor;
use crate::util::crc::compute_crc;

pub struct Decoder<'bytes> {
    compressed: &'bytes [u8],
    decompressed: &'bytes mut [u8],

    compressed_pos: usize,
    decompressed_pos: usize,

    dictionary_len: usize,
    check_crc: bool,

    header: Option<BlockHeader>,
    reset_pos: Option<usize>,
}

impl<'bytes> Decoder<'bytes> {
    pub const fn new(
        compressed: &'bytes [u8],
        decompressed: &'bytes mut [u8],
        decode_start_offset: usize,
        dictionary_len: usize,
        check_crc: bool,
    ) -> Self {
        Self {
            compressed,
            decompressed,
            compressed_pos: 0,
            decompressed_pos: decode_start_offset,
            dictionary_len,
            check_crc,
            header: None,
            reset_pos: None,
        }
    }

    pub fn decode(&mut self) -> Result<usize> {
        while self.decompressed_pos < self.decompressed.len() {
            if self.compressed.len() <= self.compressed_pos {
                return Err(Error::InvalidInput("compressed data is too short"));
            }

            self.decode_step()?;
        }

        if self.decompressed_pos != self.decompressed.len() {
            return Err(Error::DecompressionFailed);
        }

        Ok(self.decompressed_pos)
    }

    fn decode_step(&mut self) -> Result<()> {
        let mut raw_bytes_to_go = self.dictionary_len - self.decompressed_pos;
        let chunk_pos = self.decompressed_pos % BLOCK_LEN; // ?
        let raw_len_left = self.decompressed.len() - self.decompressed_pos;

        // Since `self.dictionary_len` is the same as `self.decompressed.len()` when the
        // dictionary base is not provided, is there a difference between the
        // two? Can we use `self.decompressed.len()` instead?

        raw_bytes_to_go = raw_bytes_to_go.min(BLOCK_LEN - chunk_pos);
        raw_bytes_to_go = raw_bytes_to_go.min(raw_len_left);

        if chunk_pos == 0 {
            let (header, offset) =
                BlockHeader::try_from_block(&self.compressed[self.compressed_pos..])?;

            debug_assert!(
                header.compressor() == Compressor::Kraken
                    || header.compressor() == Compressor::Mermaid
                    || header.compressor() == Compressor::Leviathan,
                "Invalid compressor"
            );

            if header.is_reset() {
                self.reset_pos = Some(self.decompressed_pos);
            }

            self.header = Some(header);
            self.compressed_pos += offset;
        }

        debug_assert!(
            self.header.is_some(),
            "The header should be initialized in the `if chunk_pos == 0` block"
        );
        let header = self.header.as_ref().expect("Header should be initialized");

        if header.is_memcpy() {
            self.copy_one(raw_bytes_to_go);
            return Ok(());
        }

        let (quantum_header, offset) = QuantumHeader::try_from(
            &self.compressed[self.compressed_pos..],
            header.has_quantum_crcs(),
            raw_bytes_to_go,
        )?;
        self.compressed_pos += offset;

        if quantum_header.compressed_len() > raw_bytes_to_go {
            return Err(Error::InvalidQuantumLength(quantum_header.compressed_len()));
        }

        if quantum_header.compressed_len() > self.compressed.len() - self.compressed_pos {
            return Err(Error::InvalidQuantumLength(quantum_header.compressed_len()));
        }

        if self.check_crc && quantum_header.compressed_len() > 0 && header.has_quantum_crcs() {
            debug_assert!(
                quantum_header.crc().is_some(),
                "if header has quantum CRCs, quantum_header should also have a CRC set"
            );
            let crc = quantum_header.crc().expect("CRC should be present");

            let computed_crc = compute_crc(
                &self.compressed
                    [self.compressed_pos..self.compressed_pos + quantum_header.compressed_len()],
            );

            if crc != computed_crc {
                return Err(Error::InvalidCRC(format!(
                    "CRC mismatch: expected {crc:08X}, got {computed_crc:08X}",
                )));
            }
        }

        if quantum_header.compressed_len() == raw_bytes_to_go {
            // memcpy

            todo!();

            // return Ok(());
        }

        self.decode_one(
            raw_bytes_to_go,
            header.compressor(),
            quantum_header.compressed_len(),
        )
    }

    fn copy_one(
        &mut self,
        raw_bytes_to_go: usize,
    ) {
        let compressed_available = self.compressed.len() - self.compressed_pos;

        if raw_bytes_to_go > compressed_available {
            return;
        }

        let src = &self.compressed[self.compressed_pos..self.compressed_pos + raw_bytes_to_go];
        let dst =
            &mut self.decompressed[self.decompressed_pos..self.decompressed_pos + raw_bytes_to_go];

        dst.copy_from_slice(src);

        self.compressed_pos += raw_bytes_to_go;
        self.decompressed_pos += raw_bytes_to_go;
    }

    fn decode_one(
        &mut self,
        raw_bytes_to_go: usize,
        block_compressor: Compressor,
        block_compressed_len: usize,
    ) -> Result<()> {
        let pos_since_reset = self.decompressed_pos - self.reset_pos.unwrap_or(0);

        let read_bytes = match block_compressor {
            Compressor::Kraken => newlz_decode_one(
                &self.compressed[self.compressed_pos..self.compressed_pos + block_compressed_len],
                &mut self.decompressed
                    [self.decompressed_pos..self.decompressed_pos + raw_bytes_to_go],
                pos_since_reset,
            )?,
            Compressor::Mermaid => newlzf_decode_one(
                &self.compressed[self.compressed_pos..self.compressed_pos + block_compressed_len],
                &mut self.decompressed
                    [self.decompressed_pos..self.decompressed_pos + raw_bytes_to_go],
                pos_since_reset,
            )?,
            Compressor::Leviathan => newlzhc_decode_one(
                &self.compressed[self.compressed_pos..self.compressed_pos + block_compressed_len],
                &mut self.decompressed
                    [self.decompressed_pos..self.decompressed_pos + raw_bytes_to_go],
                pos_since_reset,
            )?,
            Compressor::Selkie | Compressor::Hydra => unreachable!(),
        };

        self.compressed_pos += read_bytes;
        self.decompressed_pos += raw_bytes_to_go;

        if self.decompressed_pos > self.decompressed.len() {
            return Err(Error::DecompressionError(
                "Decompressed data exceeds buffer size".to_owned(),
            ));
        }

        if read_bytes != block_compressed_len {
            return Err(Error::DecompressionError(
                "Decompressed data does not match header".to_owned(),
            ));
        }

        Ok(())
    }
}
