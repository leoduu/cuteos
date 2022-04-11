
pub mod map;
pub mod consts;
pub mod rk_gpio;
pub mod rk_uart;
pub mod rk_timer;
pub mod mmu;

use core::cell::UnsafeCell;

pub use rk_gpio as gpio;
pub use rk_uart as uart;
pub use rk_timer as timer;

use crate::drivers::timer::basic_timer::BasicTimer;

const TIMER0: timer::TimerInner = unsafe { timer::TimerInner::new(0) }; 

pub const fn basic_timer() -> &'static impl BasicTimer {
    &TIMER0
}

pub fn test() -> &'static impl BasicTimer {
    &TIMER0
}


extern "Rust" {
    static __rx_start: UnsafeCell<()>;
    static __rx_end_exclusive: UnsafeCell<()>;
}

/// Start address of the Read+Execute (RX) range.
///
/// # Safety
///
/// - Value is provided by the linker script and must be trusted as-is.
#[inline(always)]
fn rx_start() -> usize {
    unsafe { __rx_start.get() as usize }
}

/// Exclusive end address of the Read+Execute (RX) range.
///
/// # Safety
///
/// - Value is provided by the linker script and must be trusted as-is.
#[inline(always)]
fn rx_end_exclusive() -> usize {
    unsafe { __rx_end_exclusive.get() as usize }
}
