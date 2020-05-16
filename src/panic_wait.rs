//! Panic handler that waits forever

use crate::cpu;
use core::panic::PanicInfo;

// We are required to define a custom panic handler if we're using #![no_std]
// ! as return type implies that this function never returns.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    cpu::wait_forever()
}
