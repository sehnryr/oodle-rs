use super::Profile;

/// Options for the compressor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CompressOptions {
    min_match_len: i32,
    seek_chunk_reset: bool,
    seek_chunk_len: i32,
    profile: Profile,
    dictionary_size: i32,
    space_speed_tradeoff_bytes: i32,
    send_quantum_crcs: bool,
    max_local_dictionary_size: i32,
    make_long_range_matcher: bool,
    match_table_size_log2: i32,
    far_match_min_len: i32,
    far_match_offset_log2: i32,
}

impl Default for CompressOptions {
    fn default() -> Self { Self::new() }
}

impl CompressOptions {
    /// Returns the default [`CompressOptions`] instance.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            min_match_len: 0,
            seek_chunk_reset: false,
            seek_chunk_len: 1 << 18,
            profile: Profile::new(),
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

    /// Creates a new [`CompressOptionsBuilder`] instance with default values.
    #[must_use]
    pub const fn builder() -> CompressOptionsBuilder { CompressOptionsBuilder::new() }

    /// Minimum match length.
    /// Cannot be used to reduce a compressor's default MML, but can be higher.
    /// On some types of data, a large MML (6 or 8) is a space-speed win.
    #[must_use]
    pub const fn min_match_len(self) -> i32 { self.min_match_len }

    /// Whether chunks should be independent, for seeking and parallelism
    #[must_use]
    pub const fn seek_chunk_reset(self) -> bool { self.seek_chunk_reset }

    /// Length of independent seek chunks (if `seek_chunk_reset`).
    /// Must be a power of 2 and >= 1<<18
    #[must_use]
    pub const fn seek_chunk_len(self) -> i32 { self.seek_chunk_len }

    /// Decoder profile to target.
    #[must_use]
    pub const fn profile(self) -> Profile { self.profile }

    /// Sets a maximum offset for matches, if lower than the maximum the format
    /// supports. <= 0 means infinite (use whole buffer). Often power of 2
    /// but doesn't have to be.
    #[must_use]
    pub const fn dictionary_size(self) -> i32 { self.dictionary_size }

    /// This is a number of bytes.
    /// I must gain at least this many bytes of compressed size to accept
    /// a speed-decreasing decision.
    #[must_use]
    pub const fn space_speed_tradeoff_bytes(self) -> i32 { self.space_speed_tradeoff_bytes }

    /// Should the encoder send a CRC of each compressed quantum, for integrity
    /// checks. This is necessary if you want to use `check_crc` on decode.
    #[must_use]
    pub const fn send_quantum_crcs(self) -> bool { self.send_quantum_crcs }

    /// Size of local dictionary before needing a long range matcher.
    /// This does not set a window size for the decoder.
    /// It's useful to limit memory use and time taken in the encoder.
    /// `max_local_dictionary_size` must be a power of 2. Must be <= 1<<30.
    #[must_use]
    pub const fn max_local_dictionary_size(self) -> i32 { self.max_local_dictionary_size }

    /// Should the encoder find matches beyond `max_local_dictionary_size` using
    /// an LRM (long range matcher)?
    #[must_use]
    pub const fn make_long_range_matcher(self) -> bool { self.make_long_range_matcher }

    /// When variable, sets the size of the match finder structure (often a hash
    /// table). Use 0 for the compressor's default.
    #[must_use]
    pub const fn match_table_size_log2(self) -> i32 { self.match_table_size_log2 }

    /// Far matches must be at least this len.
    #[must_use]
    pub const fn far_match_min_len(self) -> i32 { self.far_match_min_len }

    /// If not zero, the log2 of an offset that must meet `far_match_min_len`.
    #[must_use]
    pub const fn far_match_offset_log2(self) -> i32 { self.far_match_offset_log2 }
}

/// Builder for [`CompressOptions`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CompressOptionsBuilder(CompressOptions);

impl CompressOptionsBuilder {
    /// Creates a new [`CompressOptionsBuilder`] instance with default values.
    #[must_use]
    const fn new() -> Self { Self(CompressOptions::new()) }

    /// Builds the [`CompressOptions`] instance.
    #[must_use]
    pub const fn build(self) -> CompressOptions { self.0 }

    /// Sets the minimum match length.
    #[must_use]
    pub const fn min_match_len(
        mut self,
        min_match_len: i32,
    ) -> Self {
        self.0.min_match_len = min_match_len;
        self
    }

    /// Enables or disables seeking chunk reset.
    #[must_use]
    pub const fn seek_chunk_reset(
        mut self,
        seek_chunk_reset: bool,
    ) -> Self {
        self.0.seek_chunk_reset = seek_chunk_reset;
        self
    }

    /// Sets the length of the seeking chunk.
    #[must_use]
    pub const fn seek_chunk_len(
        mut self,
        seek_chunk_len: i32,
    ) -> Self {
        self.0.seek_chunk_len = seek_chunk_len;
        self
    }

    /// Sets the profile.
    #[must_use]
    pub const fn profile(
        mut self,
        profile: Profile,
    ) -> Self {
        self.0.profile = profile;
        self
    }

    /// Sets the dictionary size.
    #[must_use]
    pub const fn dictionary_size(
        mut self,
        dictionary_size: i32,
    ) -> Self {
        self.0.dictionary_size = dictionary_size;
        self
    }

    /// Sets the space-speed tradeoff bytes.
    #[must_use]
    pub const fn space_speed_tradeoff_bytes(
        mut self,
        space_speed_tradeoff_bytes: i32,
    ) -> Self {
        self.0.space_speed_tradeoff_bytes = space_speed_tradeoff_bytes;
        self
    }

    /// Sets whether to send quantum CRCs.
    #[must_use]
    pub const fn send_quantum_crcs(
        mut self,
        send_quantum_crcs: bool,
    ) -> Self {
        self.0.send_quantum_crcs = send_quantum_crcs;
        self
    }

    /// Sets the maximum local dictionary size.
    #[must_use]
    pub const fn max_local_dictionary_size(
        mut self,
        max_local_dictionary_size: i32,
    ) -> Self {
        self.0.max_local_dictionary_size = max_local_dictionary_size;
        self
    }

    /// Sets whether to make a long-range matcher.
    #[must_use]
    pub const fn make_long_range_matcher(
        mut self,
        make_long_range_matcher: bool,
    ) -> Self {
        self.0.make_long_range_matcher = make_long_range_matcher;
        self
    }

    /// Sets the match table size log2.
    #[must_use]
    pub const fn match_table_size_log2(
        mut self,
        match_table_size_log2: i32,
    ) -> Self {
        self.0.match_table_size_log2 = match_table_size_log2;
        self
    }

    /// Sets the far match minimum length.
    #[must_use]
    pub const fn far_match_min_len(
        mut self,
        far_match_min_len: i32,
    ) -> Self {
        self.0.far_match_min_len = far_match_min_len;
        self
    }

    /// Sets the far match offset log2.
    #[must_use]
    pub const fn far_match_offset_log2(
        mut self,
        far_match_offset_log2: i32,
    ) -> Self {
        self.0.far_match_offset_log2 = far_match_offset_log2;
        self
    }
}
