//! Some docs here

// Attributes
#![feature(format_args_nl)]
#![feature(naked_functions)]
#![feature(panic_info_message)]
#![no_main]
#![no_std]

mod bsp;
mod console;
mod cpu;
mod memory;
mod panic_wait;
mod print;
mod runtime_init;

unsafe fn kernel_init() -> ! {
    println!("[0] Hello from pure Rust!");
    println!("[1] Stopping here");
    cpu::wait_forever()
}
