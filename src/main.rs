// This is so we can use the llvm_asm marco in cpu.rs

//! Some docs here

#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(llvm_asm)]
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
    println!("[0] Hello from Rust!");
    panic!("Stopping here!")
}
