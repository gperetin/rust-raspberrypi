//! Some docs here

// Attributes
#![feature(format_args_nl)]
#![feature(naked_functions)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![no_main]
#![no_std]

mod bsp;
mod console;
mod cpu;
mod memory;
mod panic_wait;
mod print;
mod runtime_init;
mod synchronization;

unsafe fn kernel_init() -> ! {
    use console::interface::Statistics;

    println!("[0] Hello from pure Rust!");
    println!("[1] Chars written: {}", bsp::console::console().chars_written());
    println!("[2] Stopping here.");
    cpu::wait_forever()
}
