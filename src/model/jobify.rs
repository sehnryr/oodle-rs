/// Controls the amount of internal threading.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Jobify {
    /// Use compressor default for level of internal job usage
    #[default]
    Default,
    /// Try to balance parallelism with increased memory usage
    Normal,
    /// Maximize parallelism even when doing so requires large amounts of memory
    Aggressive,
}
