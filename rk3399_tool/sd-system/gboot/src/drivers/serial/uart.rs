
use crate::bsp::{UartInner, BlockingMode};
use crate::sync::Mutex;
use crate::drivers::{DeviceType, Driver, console::Console};
use core::fmt;

pub struct Uart {
    inner: Mutex<UartInner>,
}

impl Uart {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            inner: Mutex::new(UartInner::new(mmio_start_addr))
        }
    }
    
}

impl Driver for Uart {
    unsafe fn init(&self) -> Result<(), &'static str> {
        self.inner.lock(|uart| uart.init());
        Ok(())
    }

    fn get_device_type(&self) -> DeviceType {
        DeviceType::Serial
    }
}

impl Console for Uart {
    // Tx
    fn write_char(&self, ch: char) {
        // lock the uart
        self.inner.lock(|uart| uart.write(ch));
    }

    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result {
        self.inner.lock(|uart| fmt::Write::write_fmt(uart, args))
    }

    fn write_str(&self, str: &str) {
        self.inner.lock(|uart| {
            for ch in str.chars() {
                uart.write(ch);
            }
        })
    }

    fn flush(&self) {
        // lock the uart
        self.inner.lock(|uart| uart.flush())
    }

    fn write_drain(&self) {
        self.inner.lock(|uart| uart.drain())
    }

    // Rx
    fn read_char(&self) -> char {
        // lock the uart
        self.inner
            .lock(|uart| uart.read(BlockingMode::Blocking))
            .unwrap()
    }

    fn read_char_noblock(&self) -> Option<char> {
        self.inner
            .lock(|uart| uart.read(BlockingMode::NonBlocking))        
    }

    fn read_drain(&self) {
        while self.inner
            .lock(|uart| uart.read(BlockingMode::NonBlocking))
            .is_some() {

            cortex_a::asm::nop();
        }
    }
}



