//--------------------------------------------------------------------------------------------------
// Public Definitions
//--------------------------------------------------------------------------------------------------

/// Console interfaces.
pub mod interface {
    /// Console write functions.
    ///
    /// `core::fmt::Write` is exactly what we need for now. Re-export it here because
    /// implementing `console::Write` gives a better hint to the reader about the
    /// intention.
    /// pub use core::fmt::Write;
    use core::fmt;

    /// Console write functions
    pub trait Write {
        /// Write a Rust format string
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
    }

    /// Console statistics
    pub trait Statistics {
        /// Return the number of characters written
        fn chars_written(&self) -> usize {
            0
        }
    }

    /// Trait alias for full-fledged console
    pub trait All = Write + Statistics;
}