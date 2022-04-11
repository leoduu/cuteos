// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2021 Andre Richter <andre.o.richter@gmail.com>

//! Architectural timer primitives.
//!
//! # Orientation
//!
//! Since arch modules are imported into generic modules using the path attribute, the path of this
//! file is:
//!
//! crate::time::arch_time

use crate::drivers::timer::system_timer;
use crate::warn;
use core::time::Duration;
use cortex_a::{asm::barrier, registers::*};
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};
//use super::interrupt;

//--------------------------------------------------------------------------------------------------
// Private Definitions
//--------------------------------------------------------------------------------------------------

const NS_PER_S: u64 = 1_000_000_000;

/// ARMv8 Generic Timer.
pub struct GenericTimer;

static mut INTERVAL_TIME: Duration = Duration::from_nanos(0);
static mut IRQ_HANDLER: fn() = generic_timer_handler_default;

//--------------------------------------------------------------------------------------------------
// Private Code
//--------------------------------------------------------------------------------------------------

impl GenericTimer {
    #[inline(always)]
    fn read_cntpct(&self) -> u64 {
        // Prevent that the counter is read ahead of time due to out-of-order execution.
        unsafe { barrier::isb(barrier::SY) };
        CNTPCT_EL0.get()
    }

    fn set_next(&self, duration: Duration) {
        // Instantly return on zero.
        if duration.as_nanos() == 0 {
            return;
        }

        // Calculate the register compare value.
        let frq = CNTFRQ_EL0.get();
        let x = match frq.checked_mul(duration.as_nanos() as u64) {
            None => {
                warn!("Spin duration too long, skipping");
                return;
            }
            Some(val) => val,
        };
        let tval = x / NS_PER_S;

        // Set the compare value register.
        CNTP_TVAL_EL0.set(tval);

        // Enable timer and unmask interrupt.
        CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::SET + CNTP_CTL_EL0::IMASK::CLEAR);
    }

    pub fn generic_timer_handler(&self) {
        CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ISTATUS::SET + CNTP_CTL_EL0::ENABLE::SET);
        unsafe{ 
            self.set_next(INTERVAL_TIME);
            let handler: fn() = IRQ_HANDLER;
            handler();
        }
    }
}

//------------------------------------------------------------------------------
// OS Interface Code
//------------------------------------------------------------------------------

impl system_timer::Interface for GenericTimer {

    fn get_cycle(&self) -> Duration {
        
        let current_count: u128 = self.read_cntpct()as u128 * NS_PER_S as u128;
        let frq: u128 = CNTFRQ_EL0.get() as u128;
        
        Duration::from_nanos((current_count / frq) as u64)
    }

    #[inline]
    fn set_periodic(&self, duration: Duration) {
        unsafe {
            INTERVAL_TIME = duration;
        }
        self.set_next(duration);
    }

    fn init(&self) -> Result<(), &'static str> {
        CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::CLEAR);
        // unsafe {
        //     interrupt::irq_install_handler(GENERIC_TIMER_INTR, generic_timer_handler);
        // }
        // interrupt::irq_handler_enable(GENERIC_TIMER_INTR);

        Ok(())
    }

    #[inline]
    fn set_irq_handler(&self, handler: fn()) {
        unsafe { IRQ_HANDLER = handler }
    }
}

fn generic_timer_handler_default() {
    cortex_a::asm::nop();
}
