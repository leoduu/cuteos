// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2021 Andre Richter <andre.o.richter@gmail.com>

//! BSP Memory Management.

use core::cell::UnsafeCell;

//--------------------------------------------------------------------------------------------------
// Private Definitions
//--------------------------------------------------------------------------------------------------

// Symbols from the linker script.
extern "Rust" {
    static __rx_start: UnsafeCell<()>;
    static __rx_end_exclusive: UnsafeCell<()>;
}

//--------------------------------------------------------------------------------------------------
// Public Definitions
//--------------------------------------------------------------------------------------------------

/// The board's physical memory map.
#[rustfmt::skip]
/// The inclusive end address of the memory map.
///
/// End address + 1 must be power of two.
///
/// # Note
///
/// RPi3 and RPi4 boards can have different amounts of RAM. To make our code lean for
/// educational purposes, we set the max size of the address space to 4 GiB regardless of board.
/// This way, we can map the entire range that we need (end of MMIO for RPi4) in one take.
///
/// However, making this trade-off has the downside of making it possible for the CPU to assert a
/// physical address that is not backed by any DRAM (e.g. accessing an address close to 4 GiB on
/// an RPi3 that comes with 1 GiB of RAM). This would result in a crash or other kind of error.
pub const END_INCLUSIVE:       usize = 0xFFFF_FFFF;

pub const GPIO_OFFSET:         usize = 0x0020_0000;
pub const UART_OFFSET:         usize = 0x0020_1000;

/// Physical devices.
pub mod mmio {
    use super::*;

    pub const START:            usize =         0x3F00_0000;
    pub const GPIO_START:       usize = START + GPIO_OFFSET;
    pub const PL011_UART_START: usize = START + UART_OFFSET;
    pub const END_INCLUSIVE:    usize =         0x4000_FFFF;
}

//--------------------------------------------------------------------------------------------------
// Private Code
//--------------------------------------------------------------------------------------------------

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
