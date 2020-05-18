use crate::{bsp, cpu};
use cortex_a::{asm, regs::*};

// Boot Code
/// The entry of the kernel binary.
///
/// The function must be named `_start` because the linker is looking for
/// that name.
/// Linker script must place this function at 0x80_000
// Couldn't find much docs for this attribute, except https://github.com/nox/rust-rfcs/blob/master/text/1201-naked-fns.md
// It generates prologue/epilogue-free functions
#[naked]
//  Mangling is when a compiler changes the name we’ve given a function to a
//  different name that contains more information for other parts of the
//  compilation process to consume but is less human readable.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    use crate::runtime_init;

    if bsp::cpu::BOOT_CORE_ID == cpu::smp::core_id() {
        SP.set(bsp::cpu::BOOT_CORE_STACK_START);
        runtime_init::runtime_init();
    } else {
        // If not core 0, wait for events
        wait_forever()
    }
}

// Re-exporting
pub use asm::nop;

/// Spin for `n` cycles
#[inline(always)]
pub fn spin_for_cycles(n: usize) {
    for _ in 0..n {
        asm::nop();
    }
}


/// Pause execution on the core.
// This function is called by the panic handler, we could have just as easily
// defined this as a noop handler, by having just `loop {}` in the method body
#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        asm::wfe()
    }
}
