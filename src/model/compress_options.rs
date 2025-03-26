use super::Profile;

/// Options for the compressor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CompressOptions {
    /// Minimum match length.
    /// Cannot be used to reduce a compressor's default MML, but can be higher.
    /// On some types of data, a large MML (6 or 8) is a space-speed win.
    pub(crate) min_match_len: i32,
    /// Whether chunks should be independent, for seeking and parallelism
    pub(crate) seek_chunk_reset: bool,
    /// Length of independent seek chunks (if seek_chunk_reset).
    /// Must be a power of 2 and >= 1<<18
    pub(crate) seek_chunk_len: i32,
    /// Decoder profile to target.
    pub(crate) profile: Profile,
    /// Sets a maximum offset for matches, if lower than the maximum the format
    /// supports. <= 0 means infinite (use whole buffer). Often power of 2
    /// but doesn't have to be.
    pub(crate) dictionary_size: i32,
    /// This is a number of bytes.
    /// I must gain at least this many bytes of compressed size to accept
    /// a speed-decreasing decision.
    pub(crate) space_speed_tradeoff_bytes: i32,
    /// Should the encoder send a CRC of each compressed quantum, for integrity
    /// checks. This is necessary if you want to use `check_crc` on decode.
    pub(crate) send_quantum_crcs: bool,
    /// Size of local dictionary before needing a long range matcher.
    /// This does not set a window size for the decoder.
    /// It's useful to limit memory use and time taken in the encoder.
    /// `max_local_dictionary_size` must be a power of 2. Must be <= 1<<30.
    pub(crate) max_local_dictionary_size: i32,
    /// Should the encoder find matches beyond `max_local_dictionary_size` using
    /// an LRM (long range matcher)?
    pub(crate) make_long_range_matcher: bool,
    /// When variable, sets the size of the match finder structure (often a hash
    /// table). Use 0 for the compressor's default.
    pub(crate) match_table_size_log2: i32,
    /// Far matches must be at least this len.
    pub(crate) far_match_min_len: i32,
    /// If not zero, the log2 of an offset that must meet `far_match_min_len`.
    pub(crate) far_match_offset_log2: i32,
}

impl Default for CompressOptions {
    fn default() -> Self {
        Self {
            min_match_len: 0,
            seek_chunk_reset: false,
            seek_chunk_len: 1 << 18,
            profile: Profile::default(),
            dictionary_size: 0,
            space_speed_tradeoff_bytes: 256,
            send_quantum_crcs: false,
            max_local_dictionary_size: 2 << 20,
            make_long_range_matcher: true,
            match_table_size_log2: 0,
            far_match_min_len: 0,
            far_match_offset_log2: 0,
        }
    }
}

impl CompressOptions {
    /// Creates a new `CompressOptions` instance with default values.
    pub fn new() -> Self { Self::default() }

    /// Sets the minimum match length.
    pub fn min_match_len(
        mut self,
        min_match_len: i32,
    ) -> Self {
        self.min_match_len = min_match_len;
        self
    }

    /// Enables or disables seeking chunk reset.
    pub fn seek_chunk_reset(
        mut self,
        seek_chunk_reset: bool,
    ) -> Self {
        self.seek_chunk_reset = seek_chunk_reset;
        self
    }

    /// Sets the length of the seeking chunk.
    pub fn seek_chunk_len(
        mut self,
        seek_chunk_len: i32,
    ) -> Self {
        self.seek_chunk_len = seek_chunk_len;
        self
    }

    /// Sets the profile.
    pub fn profile(
        mut self,
        profile: Profile,
    ) -> Self {
        self.profile = profile;
        self
    }

    /// Sets the dictionary size.
    pub fn dictionary_size(
        mut self,
        dictionary_size: i32,
    ) -> Self {
        self.dictionary_size = dictionary_size;
        self
    }

    /// Sets the space-speed tradeoff bytes.
    pub fn space_speed_tradeoff_bytes(
        mut self,
        space_speed_tradeoff_bytes: i32,
    ) -> Self {
        self.space_speed_tradeoff_bytes = space_speed_tradeoff_bytes;
        self
    }

    /// Sets whether to send quantum CRCs.
    pub fn send_quantum_crcs(
        mut self,
        send_quantum_crcs: bool,
    ) -> Self {
        self.send_quantum_crcs = send_quantum_crcs;
        self
    }

    /// Sets the maximum local dictionary size.
    pub fn max_local_dictionary_size(
        mut self,
        max_local_dictionary_size: i32,
    ) -> Self {
        self.max_local_dictionary_size = max_local_dictionary_size;
        self
    }

    /// Sets whether to make a long-range matcher.
    pub fn make_long_range_matcher(
        mut self,
        make_long_range_matcher: bool,
    ) -> Self {
        self.make_long_range_matcher = make_long_range_matcher;
        self
    }

    /// Sets the match table size log2.
    pub fn match_table_size_log2(
        mut self,
        match_table_size_log2: i32,
    ) -> Self {
        self.match_table_size_log2 = match_table_size_log2;
        self
    }

    /// Sets the far match minimum length.
    pub fn far_match_min_len(
        mut self,
        far_match_min_len: i32,
    ) -> Self {
        self.far_match_min_len = far_match_min_len;
        self
    }

    /// Sets the far match offset log2.
    pub fn far_match_offset_log2(
        mut self,
        far_match_offset_log2: i32,
    ) -> Self {
        self.far_match_offset_log2 = far_match_offset_log2;
        self
    }
}
