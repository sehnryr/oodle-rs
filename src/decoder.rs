use crate::bindings::root::oo2::*;
use crate::error::{Error, Result};
use crate::util::compression::{compressor_scratch_memory_size, get_all_chunks_compressor};

pub struct Decoder<'a> {
    compressed: &'a [u8],
    compressed_pos: usize,
    decompressed: &'a mut [u8],
    decompressed_pos: usize,
    dictionary_len: usize,
    check_crc: bool,

    inner: OodleLZDecoder,
    _scratch: Vec<u8>,
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

        let decoder_size = size_of::<OodleLZDecoder>() + size_of::<u64>();
        let scratch_size = compressor_scratch_memory_size(compressor, decompressed.len());

        let memory_size = decoder_size + scratch_size;

        let mut scratch = vec![0u8; scratch_size];

        let decoder = OodleLZDecoder {
            decPos: decode_start_offset as i64,
            decLen: decompressed.len() as i64,
            gotHeaderPos: -1,
            resetPos: 0,
            check: 0,
            callsWithoutProgress: 0,
            ownsmem: true as i32,
            header: LZBlockHeader {
                version: 0,
                decodeType: 0,
                offsetShift: 0,
                chunkIsMemcpy: 0,
                chunkIsReset: 0,
                chunkHasQuantumCRCs: 0,
            },
            decoderSize: decoder_size as i32,
            memorySize: memory_size as i32,
            scratch: scratch.as_mut_ptr() as *mut _,
            scratch_size: scratch_size as isize,
            legacy: [0u8; 64],
        };

        Ok(Self {
            compressed,
            compressed_pos: 0,
            decompressed,
            decompressed_pos: decode_start_offset,
            dictionary_len,
            check_crc,

            inner: decoder,
            _scratch: scratch,
        })
    }

    pub fn decode(&mut self) -> Result<usize> {
        while self.decompressed_pos < self.decompressed.len() {
            let (bytes_decoded, bytes_read) = self.decode_some()?;

            self.decompressed_pos += bytes_decoded;
            self.compressed_pos += bytes_read;
        }

        if self.decompressed_pos != self.decompressed.len() {
            return Err(Error::DecompressionFailed);
        }

        Ok(self.decompressed_pos)
    }

    fn decode_some(&mut self) -> Result<(usize, usize)> {
        if self.compressed.len() <= self.compressed_pos {
            return Err(Error::InvalidInput("compressed data is too short"));
        }

        let mut out = OodleLZ_DecodeSome_Out {
            decodedCount: 0,
            compBufUsed: 0,
            curQuantumRawLen: 0,
            curQuantumCompLen: 0,
        };

        let result = unsafe {
            OodleLZDecoder_DecodeSome(
                std::ptr::addr_of_mut!(self.inner) as *mut _,
                &mut out,
                self.decompressed.as_mut_ptr() as *mut _,
                self.decompressed_pos as isize,
                self.dictionary_len as isize,
                (self.dictionary_len - self.decompressed_pos) as isize,
                self.compressed.as_ptr().add(self.compressed_pos) as *const _,
                (self.compressed.len() - self.compressed_pos) as isize,
                OodleLZ_FuzzSafe::OodleLZ_FuzzSafe_Yes,
                if self.check_crc {
                    OodleLZ_CheckCRC::OodleLZ_CheckCRC_Yes
                } else {
                    OodleLZ_CheckCRC::OodleLZ_CheckCRC_No
                },
                OodleLZ_Verbosity::OodleLZ_Verbosity_None,
                OodleLZ_Decode_ThreadPhase::OodleLZ_Decode_ThreadPhaseAll,
            )
        };

        // result is bool
        if result == 0 {
            return Err(Error::DecompressionFailed);
        }

        if out.decodedCount == 0 {
            return Err(Error::DecompressionFailed);
        }

        Ok((out.decodedCount as usize, out.compBufUsed as usize))
    }
}
