/// Console interfaces
pub mod interface {
    use core::fmt;

    /// Console write functions
    ///
    /// Re-exporting here because `console::Write` gives a better hint of the intention
    pub trait Write {
        /// Write a single character
        fn write_char(&self, c: char);
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
    }

    /// Console read functions
    pub trait Read {
        /// Read a single character
        fn read_char(&self) -> char {
            ' '
        }
    }

    /// Console statistics.
    pub trait Statistics {
        /// Return the number of characters written
        fn chars_written(&self) -> usize {
            0
        }

        /// Return the number of characters read
        fn chars_read(&self) -> usize {
            0
        }
    }

    /// Trait alias for a full-fledged console
    pub trait All = Write + Read + Statistics;
}

