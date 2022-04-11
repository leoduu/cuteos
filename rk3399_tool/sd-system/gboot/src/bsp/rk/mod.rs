
pub mod map;
mod rk_gpio;
mod rk_uart;
mod rk_timer;

pub use rk_timer::*;
pub use rk_gpio::*;
pub use rk_uart::*;

use core::cell::UnsafeCell;

extern "Rust" {
    static __rx_start: UnsafeCell<()>;
    static __rx_end: UnsafeCell<()>;
}
