/// Compression encoder complexity.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionLevel {
    /// Super fast mode, lower compression ratio
    SuperFast,
    /// Fastest LZ mode with still decent compression ratio
    VeryFast,
    /// Fast - good for daily use
    Fast,
    /// Standard medium speed LZ mode
    #[default]
    Normal,

    /// Optimal parse level 1 (faster optimal encoder)
    Optimal1,
    /// Optimal parse level 2 (recommended baseline optimal encoder)
    Optimal2,
    /// Optimal parse level 3 (slower optimal encoder)
    Optimal3,
    /// Optimal parse level 4 (very slow optimal encoder)
    Optimal4,
    /// Optimal parse level 5 (don't care about encode speed, maximum compression)
    Optimal5,

    /// Faster than SuperFast, less compression
    HyperFast1,
    /// Faster than HyperFast1, less compression
    HyperFast2,
    /// Faster than HyperFast2, less compression
    HyperFast3,
    /// Fastest, less compression
    HyperFast4,
}

impl CompressionLevel {
    pub const HYPER_FAST: CompressionLevel = CompressionLevel::HyperFast1;
    pub const OPTIMAL: CompressionLevel = CompressionLevel::Optimal2;
    pub const MAX: CompressionLevel = CompressionLevel::Optimal5;
    pub const MIN: CompressionLevel = CompressionLevel::HyperFast4;
}
