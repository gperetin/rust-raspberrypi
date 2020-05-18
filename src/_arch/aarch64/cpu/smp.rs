//! Architectural symmetric multiprocessing

use cortex_a::regs::*;

/// Return the executing core's id
#[inline(always)]
pub fn core_id<T>() -> T
where
    T: From<u8>,
{
    // We need only 0 and 1 bits
    const CORE_MASK: u64 = 0b11;
    T::from((MPIDR_EL1.get() & CORE_MASK) as u8)
}
