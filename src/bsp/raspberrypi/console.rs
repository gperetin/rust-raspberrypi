// We import this because we want the interface
use crate::{console, synchronization, synchronization::NullLock};
use core::fmt;


struct QEMUOutputInner {
    chars_written: usize,
}


pub struct QEMUOutput {
    inner: NullLock<QEMUOutputInner>,
}

static QEMU_OUTPUT: QEMUOutput = QEMUOutput::new();

impl QEMUOutputInner {
    const fn new() -> QEMUOutputInner {
        QEMUOutputInner { chars_written: 0 }
    }

    /// Send a character
    fn write_char(&mut self, c: char) {
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
        self.chars_written += 1;
    }
}

// Implementing Write here enables usage of `format_args!` macros, which are used
// to implement print! and println! macros
impl fmt::Write for QEMUOutputInner {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            // Convert newline to carriage return + newline
            if c == '\n' {
                self.write_char('\r');
            }
            self.write_char(c)
        }

        Ok(())
    }
}

impl QEMUOutput {
    /// Create a new instance.
    pub const fn new() -> QEMUOutput {
        QEMUOutput {
            inner: NullLock::new(QEMUOutputInner::new()),
        }
    }
}

pub fn console() -> &'static impl console::interface::All {
    &QEMU_OUTPUT
}

use synchronization::interface::Mutex;

/// Passthrough of `args` to the `core::fmt::Write` implementation, but guarded by a Mutex to
/// serialize access.
impl console::interface::Write for QEMUOutput {
    fn write_fmt(&self, args: core::fmt::Arguments) -> fmt::Result {
        // Fully qualified syntax for the call to `core::fmt::Write::write:fmt()` to increase
        // readability.
        let mut r = &self.inner;
        r.lock(|inner| fmt::Write::write_fmt(inner, args))
    }
}

impl console::interface::Statistics for QEMUOutput {
    fn chars_written(&self) -> usize {
        let mut r = &self.inner;
        r.lock(|inner| inner.chars_written)
    }
}
