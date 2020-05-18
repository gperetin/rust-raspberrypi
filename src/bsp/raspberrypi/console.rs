// We import this because we want the interface
use crate::console;
use core::fmt;

struct QEMUOutput;

// Implementing Write here enables usage of `format_args!` macros, which are used
// to implement print! and println! macros
impl fmt::Write for QEMUOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            unsafe {
                // It is surprisingly difficult to find the base address mappings for
                // Raspberry Pi, and the doc that is supposed to have it
                // https://www.raspberrypi.org/app/uploads/2012/02/BCM2835-ARM-Peripherals.pdf
                // says the base address for UART is 0x7E20100.
                // Per the comment in the doc above:
                // Peripherals (at physical address 0x20000000 on) are mapped into the
                // kernel virtual address space starting at address 0xF2000000. Thus a
                // peripheral advertised here at bus address 0x7Ennnnnn is available in
                // the ARM kenel at virtual address 0xF2nnnnnn
                // I think there's a 0 missing in the address that's presented in that
                // doc, which is 0x7E20100
                // This https://www.raspberrypi.org/forums/viewtopic.php?p=941738&sid=c3cd9393565bb1c153827927ae8da5fc#p941738
                // blog post says the peripheral base address is 0x3F000000
                // So, to wrap up, I think if we add the 0 to the incorrect address from the
                // doc, and use base peripheral address of 0x3F000000, and then do the
                // replacement, described above, we get to this address
                core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
            }
        }

        Ok(())
    }
}

pub fn console() -> impl console::interface::Write {
    QEMUOutput {}
}
