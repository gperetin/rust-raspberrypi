/// Console interfaces
pub mod interface {
    /// Console write functions
    ///
    /// Re-exporting here because `console::Write` gives a better hint of the intention
    pub use core::fmt::Write;
}
