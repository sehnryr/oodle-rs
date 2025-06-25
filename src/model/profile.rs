/// Decode profile to target.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Profile {
    /// Main profile (all current features allowed)
    #[default]
    Main,
    /// Reduced profile (Kraken only, limited feature set)
    Reduced,
}

impl Profile {
    /// Returns the default profile.
    #[must_use]
    pub const fn new() -> Self { Self::Main }
}
