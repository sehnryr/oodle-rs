#[cfg(feature = "cold_path")]
use std::hint::cold_path;

use crate::BLOCK_LEN;
use crate::bindings::root::oo2::*;
use crate::error::{Error, Result};
use crate::header::{BlockHeader, QuantumHeader};
use crate::model::Compressor;
use crate::util::compression::{compressor_scratch_memory_size, get_all_chunks_compressor};
use crate::util::crc::compute_crc;

pub struct Decoder<'a> {
    compressor: Compressor,

    compressed: &'a [u8],
    decompressed: &'a mut [u8],

    compressed_pos: usize,
    decompressed_pos: usize,

    dictionary_len: usize,
    check_crc: bool,

    header: Option<BlockHeader>,
    reset_pos: Option<usize>,
}

impl<'a> Decoder<'a> {
    pub fn new(
        compressed: &'a [u8],
        decompressed: &'a mut [u8],
        decode_start_offset: usize,
        dictionary_len: usize,
        check_crc: bool,
    ) -> Result<Self> {
        let compressor = get_all_chunks_compressor(&compressed, decompressed.len())?;

        Ok(Self {
            compressor,
            compressed,
            decompressed,
            compressed_pos: 0,
            decompressed_pos: decode_start_offset,
            dictionary_len,
            check_crc,
            header: None,
            reset_pos: None,
        })
    }

    pub fn decode(&mut self) -> Result<usize> {
        while self.decompressed_pos < self.decompressed.len() {
            self.decode_some()?;
        }

        if self.decompressed_pos != self.decompressed.len() {
            return Err(Error::DecompressionFailed);
        }

        Ok(self.decompressed_pos)
    }

    fn decode_some(&mut self) -> Result<()> {
        if self.compressed.len() <= self.compressed_pos {
            return Err(Error::InvalidInput("compressed data is too short"));
        }

        let mut raw_bytes_to_go = self.dictionary_len - self.decompressed_pos;
        let chunk_pos = self.decompressed_pos % BLOCK_LEN; // ?
        let raw_len_left = self.decompressed.len() - self.decompressed_pos;

        // Since `self.dictionary_len` is the same as `self.decompressed.len()` when the dictionary
        // base is not provided, is there a difference between the two?
        // Can we use `self.decompressed.len()` instead?

        raw_bytes_to_go = raw_bytes_to_go.min(BLOCK_LEN - chunk_pos);
        raw_bytes_to_go = raw_bytes_to_go.min(raw_len_left);

        if raw_bytes_to_go == 0 {
            #[cfg(feature = "cold_path")]
            cold_path();

            return Ok(());
        }

        if self.compressed.len() == self.compressed_pos {
            #[cfg(feature = "cold_path")]
            cold_path();

            return Ok(());
        }

        if chunk_pos == 0 {
            let (header, offset) =
                BlockHeader::try_from_block(&self.compressed[self.compressed_pos..])?;

            debug_assert!(
                header.compressor == Compressor::Kraken
                    || header.compressor == Compressor::Mermaid
                    || header.compressor == Compressor::Leviathan
            );

            if header.is_reset {
                self.reset_pos = Some(self.decompressed_pos);
            }

            self.header = Some(header);
            self.compressed_pos += offset;
        }

        debug_assert!(self.header.is_some());
        let header = self.header.as_ref().unwrap();

        if header.is_memcpy {
            let compressed_available = self.compressed.len() - self.compressed_pos;

            if raw_bytes_to_go > compressed_available {
                #[cfg(feature = "cold_path")]
                cold_path();

                return Ok(());
            }

            let src = &self.compressed[self.compressed_pos..self.compressed_pos + raw_bytes_to_go];
            let dst = &mut self.decompressed
                [self.decompressed_pos..self.decompressed_pos + raw_bytes_to_go];

            dst.copy_from_slice(src);

            self.compressed_pos += raw_bytes_to_go;
            self.decompressed_pos += raw_bytes_to_go;

            return Ok(());
        }

        let (quantum_header, offset) = QuantumHeader::try_from(
            &self.compressed[self.compressed_pos..],
            header.has_quantum_crcs,
            raw_bytes_to_go,
        )?;
        self.compressed_pos += offset;

        if quantum_header.compressed_len > raw_bytes_to_go {
            #[cfg(feature = "cold_path")]
            cold_path();

            return Err(Error::InvalidQuantumLength(quantum_header.compressed_len));
        }

        if quantum_header.compressed_len > self.compressed.len() - self.compressed_pos {
            #[cfg(feature = "cold_path")]
            cold_path();

            return Err(Error::InvalidQuantumLength(quantum_header.compressed_len));
        }

        if self.check_crc && quantum_header.compressed_len > 0 && header.has_quantum_crcs {
            #[cfg(feature = "cold_path")]
            cold_path();

            debug_assert!(quantum_header.crc.is_some());
            let crc = quantum_header.crc.unwrap();

            let computed_crc = compute_crc(
                &self.compressed
                    [self.compressed_pos..self.compressed_pos + quantum_header.compressed_len],
            );

            if crc != computed_crc {
                return Err(Error::InvalidCRC(format!(
                    "CRC mismatch: expected {:08X}, got {:08X}",
                    crc, computed_crc
                )));
            }
        }

        if quantum_header.compressed_len == raw_bytes_to_go {
            #[cfg(feature = "cold_path")]
            cold_path();

            // memcpy

            todo!();

            // return Ok(());
        }

        let pos_since_reset = self.decompressed_pos - self.reset_pos.unwrap_or(0);

        let scratch_size = compressor_scratch_memory_size(self.compressor, self.decompressed.len());
        let mut scratch = vec![0u8; scratch_size];

        let (written_bytes, read_bytes) = match header.compressor {
            Compressor::Kraken => kraken_decode_one_quantum(
                &self.compressed
                    [self.compressed_pos..self.compressed_pos + quantum_header.compressed_len],
                &mut self.decompressed
                    [self.decompressed_pos..self.decompressed_pos + raw_bytes_to_go],
                pos_since_reset,
                &mut scratch,
            )?,
            Compressor::Leviathan => leviathan_decode_one_quantum(
                &self.compressed
                    [self.compressed_pos..self.compressed_pos + quantum_header.compressed_len],
                &mut self.decompressed
                    [self.decompressed_pos..self.decompressed_pos + raw_bytes_to_go],
                pos_since_reset,
                &mut scratch,
            )?,
            Compressor::Mermaid => mermaid_decode_one_quantum(
                &self.compressed
                    [self.compressed_pos..self.compressed_pos + quantum_header.compressed_len],
                &mut self.decompressed
                    [self.decompressed_pos..self.decompressed_pos + raw_bytes_to_go],
                pos_since_reset,
                &mut scratch,
            )?,
            Compressor::Selkie | Compressor::Hydra => unreachable!(),
        };

        self.compressed_pos += read_bytes;
        self.decompressed_pos += written_bytes;

        if self.decompressed_pos > self.decompressed.len() {
            #[cfg(feature = "cold_path")]
            cold_path();

            return Err(Error::DecompressionError(
                "Decompressed data exceeds buffer size".to_owned(),
            ));
        }

        if read_bytes != quantum_header.compressed_len {
            #[cfg(feature = "cold_path")]
            cold_path();

            return Err(Error::DecompressionError(
                "Decompressed data does not match header".to_owned(),
            ));
        }

        Ok(())
    }
}

fn kraken_decode_one_quantum(
    compressed: &[u8],
    decompressed: &mut [u8],
    pos_since_reset: usize,
    scratch: &mut [u8],
) -> Result<(usize, usize)> {
    let base_decompressed_ptr = decompressed.as_mut_ptr();
    let decompressed_ptr = base_decompressed_ptr.clone();

    let read_bytes = unsafe {
        Kraken_DecodeOneQuantum(
            decompressed_ptr,
            decompressed_ptr.add(decompressed.len()),
            compressed.as_ptr(),
            compressed.len() as i32,
            compressed.as_ptr().add(compressed.len()),
            pos_since_reset as isize,
            scratch.as_mut_ptr() as *mut _,
            scratch.len() as isize,
            OodleLZ_Decode_ThreadPhase::OodleLZ_Decode_ThreadPhaseAll,
        ) as usize
    };

    if read_bytes == 0 {
        #[cfg(feature = "cold_path")]
        cold_path();

        return Err(Error::DecompressionFailed);
    }

    if read_bytes != compressed.len() {
        #[cfg(feature = "cold_path")]
        cold_path();

        return Err(Error::DecompressionError(format!(
            "Decompressed data does not match header: {} != {}",
            read_bytes,
            compressed.len()
        )));
    }

    let written_bytes = decompressed.len();

    Ok((written_bytes, read_bytes))
}

fn leviathan_decode_one_quantum(
    compressed: &[u8],
    decompressed: &mut [u8],
    pos_since_reset: usize,
    scratch: &mut [u8],
) -> Result<(usize, usize)> {
    let base_decompressed_ptr = decompressed.as_mut_ptr();
    let decompressed_ptr = base_decompressed_ptr.clone();

    let read_bytes = unsafe {
        Leviathan_DecodeOneQuantum(
            decompressed_ptr,
            decompressed_ptr.add(decompressed.len()),
            compressed.as_ptr(),
            compressed.len() as i32,
            compressed.as_ptr().add(compressed.len()),
            pos_since_reset as isize,
            scratch.as_mut_ptr() as *mut _,
            scratch.len() as isize,
            OodleLZ_Decode_ThreadPhase::OodleLZ_Decode_ThreadPhaseAll,
        ) as usize
    };

    if read_bytes == 0 {
        #[cfg(feature = "cold_path")]
        cold_path();

        return Err(Error::DecompressionFailed);
    }

    if read_bytes != compressed.len() {
        #[cfg(feature = "cold_path")]
        cold_path();

        return Err(Error::DecompressionError(format!(
            "Decompressed data does not match header: {} != {}",
            read_bytes,
            compressed.len()
        )));
    }

    let written_bytes = decompressed.len();

    Ok((written_bytes, read_bytes))
}

fn mermaid_decode_one_quantum(
    compressed: &[u8],
    decompressed: &mut [u8],
    pos_since_reset: usize,
    scratch: &mut [u8],
) -> Result<(usize, usize)> {
    let base_decompressed_ptr = decompressed.as_mut_ptr();
    let decompressed_ptr = base_decompressed_ptr.clone();

    let read_bytes = unsafe {
        Mermaid_DecodeOneQuantum(
            decompressed_ptr,
            decompressed_ptr.add(decompressed.len()),
            compressed.as_ptr(),
            compressed.len() as i32,
            compressed.as_ptr().add(compressed.len()),
            pos_since_reset as isize,
            scratch.as_mut_ptr() as *mut _,
            scratch.len() as isize,
            OodleLZ_Decode_ThreadPhase::OodleLZ_Decode_ThreadPhaseAll,
        ) as usize
    };

    if read_bytes == 0 {
        #[cfg(feature = "cold_path")]
        cold_path();

        return Err(Error::DecompressionFailed);
    }

    if read_bytes != compressed.len() {
        #[cfg(feature = "cold_path")]
        cold_path();

        return Err(Error::DecompressionError(format!(
            "Decompressed data does not match header: {} != {}",
            read_bytes,
            compressed.len()
        )));
    }

    let written_bytes = decompressed.len();

    Ok((written_bytes, read_bytes))
}
