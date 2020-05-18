//! Panic handler that waits forever

use crate::{bsp, cpu};
use core::{fmt, panic::PanicInfo};

//--------------------------------------------------------------------------------------------------
// Private Code
//--------------------------------------------------------------------------------------------------

fn _panic_print(args: fmt::Arguments) {
    use fmt::Write;

    unsafe { bsp::console::panic_console_out().write_fmt(args).unwrap() };
}

/// Prints with a newline - only use from the panic handler.
///
/// Carbon copy from https://doc.rust-lang.org/src/std/macros.rs.html
#[macro_export]
macro_rules! panic_println {
    ($($arg:tt)*) => ({
        _panic_print(format_args_nl!($($arg)*));
    })
}

// We are required to define a custom panic handler if we're using #![no_std]
// ! as return type implies that this function never returns.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(args) = info.message() {
        panic_println!("\nKernel panic: {}", args);
    } else {
        panic_println!("\nKernel panic!");
    }
    cpu::wait_forever()
}
