// This is so we can use the llvm_asm marco in cpu.rs

//! Some docs here

#![feature(global_asm)]
#![feature(llvm_asm)]
#![no_main]
#![no_std]

mod bsp;
mod cpu;
mod memory;
mod panic_wait;
mod runtime_init;

unsafe fn kernel_init() -> ! {
    panic!()
}
