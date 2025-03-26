/// Compression algorithm.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compressor {
    /// Fast decompression and high compression ratios, amazing!
    #[default]
    Kraken,
    /// Leviathan = Kraken's big brother with higher compression, slightly
    /// slower decompression.
    Leviathan,
    /// Mermaid is between Kraken & Selkie - crazy fast, still decent
    /// compression.
    Mermaid,
    /// Selkie is a super-fast relative of Mermaid.  For maximum decode speed.
    Selkie,
    /// Hydra, the many-headed beast = Leviathan, Kraken, Mermaid, or Selkie
    Hydra,
}
