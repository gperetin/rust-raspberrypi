use crate::{bsp, console};
use core::fmt;

// This doc attribute means this item won't appear in the documentation
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    // TODO: why do we need this use here?
    use console::interface::Write;

    bsp::console::console().write_fmt(args).unwrap();
}

/// Print without a new line
// Copied from https://doc.rust-lang.org/src/std/macros.rs.html
// macro_export is so that we can use the macro in other modules
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

/// Print with a new line
// Copied from https://doc.rust-lang.org/src/std/macros.rs.html
// macro_export is so that we can use the macro in other modules
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::print::_print(format_args_nl!($($arg)*));
    })
}
