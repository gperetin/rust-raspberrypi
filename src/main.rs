// This is so we can use the llvm_asm marco in cpu.rs

//! Some docs here

#![feature(global_asm)]
#![feature(llvm_asm)]
#![no_main]
#![no_std]

mod bsp;
mod cpu;
mod panic_wait;
