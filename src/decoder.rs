#[cfg(feature = "cold_path")]
use std::hint::cold_path;

use crate::bindings::root::oo2::*;
use crate::error::{
    Error,
    Result,
};
use crate::header::{
    BlockHeader,
    QuantumHeader,
};
use crate::model::Compressor;
use crate::util::compression::{
    compressor_scratch_memory_size,
    get_all_chunks_compressor,
};
use crate::util::crc::compute_crc;
use crate::{
    BLOCK_LEN,
    CHUNK_LEN,
    MIN_CHUNK_LEN,
};

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

        // Since `self.dictionary_len` is the same as `self.decompressed.len()` when the
        // dictionary base is not provided, is there a difference between the
        // two? Can we use `self.decompressed.len()` instead?

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

        let read_bytes = match header.compressor {
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
        self.decompressed_pos += raw_bytes_to_go;

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
) -> Result<usize> {
    debug_assert!(decompressed.len() <= BLOCK_LEN);

    let mut compressed_pos = 0;
    let mut decompressed_pos = 0;

    while decompressed_pos < decompressed.len() {
        let chunk_len = (decompressed.len() - decompressed_pos).min(CHUNK_LEN);
        let chunk_pos = decompressed_pos + pos_since_reset;

        debug_assert!(chunk_len > 0);
        debug_assert!(chunk_len <= CHUNK_LEN);

        // Minimum length of a chunk is 4 bytes:
        // - 3 bytes for the header
        // - 1 byte for the payload
        if compressed.len() - compressed_pos < 4 {
            return Err(Error::InvalidCompressedData(
                "Not enough data to read chunk header",
            ));
        }

        let mut chunk_compressed_len = u32::from_be_bytes([
            0,
            compressed[compressed_pos],
            compressed[compressed_pos + 1],
            compressed[compressed_pos + 2],
        ]) as usize;
        debug_assert!(chunk_compressed_len >= 1 << 23);

        let chunk_type = match chunk_compressed_len >> 19 & 0xF {
            chunk_type @ (0 | 1) => chunk_type,
            _ => return Err(Error::InvalidCompressedData("Invalid chunk type")),
        };
        chunk_compressed_len &= (1 << 19) - 1;

        compressed_pos += 3;

        if chunk_compressed_len > compressed.len() - compressed_pos {
            return Err(Error::InvalidCompressedData(
                "Chunk compressed length exceeds available data",
            ));
        }
        if chunk_compressed_len > chunk_len {
            return Err(Error::InvalidCompressedData(
                "Chunk compressed length exceeds chunk length",
            ));
        }

        // Raw chunk
        if chunk_compressed_len == chunk_len {
            if chunk_type != 0 {
                return Err(Error::InvalidCompressedData("Chunk type is not raw"));
            }

            decompressed[decompressed_pos..decompressed_pos + chunk_len]
                .copy_from_slice(&compressed[compressed_pos..compressed_pos + chunk_len]);
        } else {
            if chunk_len < MIN_CHUNK_LEN {
                return Err(Error::InvalidCompressedData("Chunk length is too short"));
            }

            debug_assert!(
                scratch.len() >= compressor_scratch_memory_size(Compressor::Kraken, chunk_len)
            );

            // This is normally created from the scratch memory,
            // but I don't want to use scratch memory for this rust reimplementation
            let mut chunk_arrays = newLZ_chunk_arrays {
                chunk_ptr: std::ptr::null_mut(),
                scratch_ptr: std::ptr::null_mut(),
                offsets: std::ptr::null_mut(),
                offsets_count: 0,
                excesses: std::ptr::null_mut(),
                excesses_count: 0,
                packets: std::ptr::null_mut(),
                packets_count: 0,
                literals_ptr: std::ptr::null_mut(),
                literals_count: 0,
            };

            unsafe {
                let compressed_ptr = compressed.as_ptr().add(compressed_pos);
                let decompressed_ptr = decompressed.as_mut_ptr().add(decompressed_pos);
                let scratch_ptr = scratch.as_mut_ptr();

                newLZ_decode_chunk_phase1(
                    chunk_type as i32,
                    compressed_ptr,
                    compressed_ptr.add(chunk_compressed_len),
                    decompressed_ptr,
                    chunk_len as isize,
                    chunk_pos as isize,
                    scratch_ptr,
                    scratch_ptr.add(scratch.len()),
                    &mut chunk_arrays,
                ) as usize;

                debug_assert!(chunk_arrays.chunk_ptr == decompressed_ptr);
                debug_assert!(chunk_arrays.scratch_ptr == scratch_ptr);

                newLZ_decode_chunk_phase2(
                    chunk_type as i32,
                    decompressed_ptr,
                    chunk_len as isize,
                    chunk_pos as isize,
                    &mut chunk_arrays,
                );
            };
        }

        compressed_pos += chunk_compressed_len;
        decompressed_pos += chunk_len;
    }

    Ok(compressed_pos)
}

fn leviathan_decode_one_quantum(
    compressed: &[u8],
    decompressed: &mut [u8],
    pos_since_reset: usize,
    scratch: &mut [u8],
) -> Result<usize> {
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

    Ok(read_bytes)
}

fn mermaid_decode_one_quantum(
    compressed: &[u8],
    decompressed: &mut [u8],
    pos_since_reset: usize,
    scratch: &mut [u8],
) -> Result<usize> {
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

    Ok(read_bytes)
}
