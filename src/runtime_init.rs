use crate::memory; // crate here refers to the current crate
use core::ops::Range;

/// Return the range spanning the .bss section
/// # Safety
///
/// - The symbol-provided addresses must be valid.
/// - The symbol-provided addresses must be usize aligned.
// unsafe is required here both because of our use of FFI, as well as having
// mutable static variables
unsafe fn bss_range() -> Range<*mut usize> {
    // This is a for of FFI (Foreign Function Interface)
    // We're basically declaring here that we will refer to these 2 variables
    // from Rust code.
    // More docs: https://doc.rust-lang.org/reference/items/external-blocks.html#statics
    extern "C" {
        // Boundaries of the .bss section, provided by the linker script symbols
        // static in Rust represents a precise memory location - all references to a
        // static variable refer to the same memory location.
        // Static items have `static` lifetime.
        // Docs https://doc.rust-lang.org/reference/items/static-items.html#mutable-statics
        static mut __bss_start: usize;
        static mut __bss_end: usize;
    }

    Range {
        start: &mut __bss_start,
        end: &mut __bss_end,
    }
}

/// Zero out the .bss section
///
/// # Safety
///
/// - Must only be called pre `kernel_init()`
#[inline(always)]
unsafe fn zero_bss() {
    memory::zero_volatile(bss_range());
}


/// Clears the .bss section, then jumps to kernel ini code
///
/// # Safety
///
/// - Only a single core must be active and running this function
#[no_mangle]
pub unsafe extern "C" fn runtime_init() -> ! {
    zero_bss();
    crate::kernel_init()
}
