// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2021 Andre Richter <andre.o.richter@gmail.com>

//! Top-level BSP file for the Raspberry Pi

pub mod console;
//pub mod mmu;
pub mod map;

use crate::drivers::{Driver, DriverManager};
use crate::drivers::timer::system_timer;
use super::devices_driver::arm;
use super::devices_driver::bcm;


//--------------------------------------------------------------------------------------------------
// Driver lists
//--------------------------------------------------------------------------------------------------

static GPIO: bcm::GPIO =
    unsafe { bcm::GPIO::new(map::mmio::GPIO_START) };

static PL011_UART: bcm::PL011Uart =
    unsafe { bcm::PL011Uart::new(map::mmio::PL011_UART_START) };

static GENERIC_TIMER: arm::GenericTimer = arm::GenericTimer;

//--------------------------------------------------------------------------------------------------
// Driver Manager
//--------------------------------------------------------------------------------------------------

static DRIVER_MANAGER: Raspi3DriverManager = Raspi3DriverManager {
    drivers: [
        &self::GPIO,
        &self::PL011_UART
    ]
};

struct Raspi3DriverManager {
    drivers: [&'static (dyn Driver + Sync); 2]
}

pub fn driver_manager() -> &'static impl DriverManager {
    &DRIVER_MANAGER
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

impl DriverManager for Raspi3DriverManager {
    fn all_device_drivers(&self) -> &[&'static (dyn Driver + Sync)] {
        &self.drivers[..]
    }

    fn post_device_driver_init(&self) {
        GPIO.map_pl011_uart()
    }
}

/// Board identification.
pub fn board_name() -> &'static str {
        "RaspberryPi 3"
}



pub fn generic_timer() -> &'static impl system_timer::Interface {
    &GENERIC_TIMER
}

const GENERIC_TIMER_INTR: usize = 30;

