/// Console interfaces
pub mod interface {
    use core::fmt;

    /// Console write functions
    ///
    /// Re-exporting here because `console::Write` gives a better hint of the intention
    pub trait Write {
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
    }

    /// Console statistics.
    pub trait Statistics {
        /// Return the number of characters_written
        fn chars_written(&self) -> usize {
            0
        }
    }

    /// Trait alias for a full-fledged console
    pub trait All = Write + Statistics;
}

