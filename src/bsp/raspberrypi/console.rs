// We import this because we want the interface
use super::memory;
use crate::{bsp::device_driver, console};
use core::fmt;


/// Use only for printing during a panic
pub unsafe fn panic_console_out() -> impl fmt::Write {
    let mut uart = device_driver::PanicUart::new(memory::map::mmio::PL011_UART_BASE);
    uart.init();
    uart
}

pub fn console() -> &'static impl console::interface::All {
    &super::PL011_UART
}
