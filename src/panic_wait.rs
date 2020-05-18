//! Panic handler that waits forever

use crate::{cpu, println};
use core::panic::PanicInfo;

// We are required to define a custom panic handler if we're using #![no_std]
// ! as return type implies that this function never returns.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(args) = info.message() {
        println!("\nKernel panic: {}", args);
    } else {
        println!("\nKernel panic!");
    }
    cpu::wait_forever()
}
