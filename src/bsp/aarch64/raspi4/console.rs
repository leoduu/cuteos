// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2021 Andre Richter <andre.o.richter@gmail.com>

//! BSP console facilities.

use super::map;
use crate::drivers::console;
use core::fmt;
use crate::bsp::devices_driver::bcm;

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

pub unsafe fn panic_console_out() -> impl fmt::Write {
    let mut panic_gpio = bcm::PanicGPIO::new(map::mmio::GPIO_START);
    let mut panic_uart = bcm::PanicUart::new(map::mmio::PL011_UART_START);

    panic_gpio.map_pl011_uart();
    panic_uart.init();
    panic_uart
}

/// Return a reference to the console.
pub fn console() -> &'static impl console::Interface {
    &super::PL011_UART
}
