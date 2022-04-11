
use crate::bsp::{TimerInner, BlockingMode};
use crate::sync::Mutex;
use crate::drivers::{DeviceType, Driver, console::Console};
use core::fmt;

pub struct Timer {
    inner: Mutex<TimerInner>,
}

impl Timer {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            inner: Mutex::new(TimerInner::new(mmio_start_addr))
        }
    }
}

impl Timer {
    // pub fn delay_ns(&self, cnt: u64) {
    //     self.inner.lock(|timer| timer.delay_ns(cnt))
    // }

    pub fn delay_us(&self, cnt: u64) {
        self.inner.lock(|timer| timer.delay_us(cnt))
    }

    pub fn delay_ms(&self, cnt: u64) {
        self.inner.lock(|timer| timer.delay_us(cnt * 1000))
    }

    pub fn irq_delay_us(&self, cnt: u64) {
        self.inner.lock(|timer| timer.irq_delay_us(cnt))
    }

    pub fn clear_irq(&self) {
        self.inner.lock(|timer| timer.clear_irq())
    }

    pub fn current_val(&self) -> u64 {
        self.inner.lock(|timer| timer.current_val())
    }
}

impl Driver for Timer {
    unsafe fn init(&self) -> Result<(), &'static str> {
        self.inner.lock(|timer| timer.init());
        Ok(())
    }

    fn get_device_type(&self) -> DeviceType {
        DeviceType::Timer
    }

}



