// This includes the assembly version of this file
global_asm!(include_str!("cpu.S"));

/// Pause execution on the core.
// This function is called by the panic handler, we could have just as easily
// defined this as a noop handler, by having just `loop {}` in the method body
#[inline(always)]
pub fn wait_forever() -> ! {
    unsafe {
        loop {
            // This macro is documented here:
            // https://doc.rust-lang.org/nightly/unstable-book/library-features/llvm-asm.html
            // First (and only required) argument is called assembly template and I think it
            // represents the command, or a piece of assemby code to execute.
            llvm_asm!("wfe"
                :               // outputs
                :               // inputs
                :               // clobbers
                // volatile - specifying this is analogous to __asm__ __volatile__ (...) in gcc/clang.
                : "volatile")   // options
        }
    }
}
