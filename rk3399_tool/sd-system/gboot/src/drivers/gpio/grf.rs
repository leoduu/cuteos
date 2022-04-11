use crate::bsp::GPIOGRFInner;
use crate::sync::Mutex;
use crate::drivers::{Driver, DeviceType};

pub struct GPIOGRF {
    inner: Mutex<GPIOGRFInner>
}

impl GPIOGRF {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            inner: Mutex::new(GPIOGRFInner::new(mmio_start_addr))
        }
    }

    pub fn map_uart2(&self) {
        self.inner.lock(|grf| grf.map_uart2());
    }
}

impl Driver for GPIOGRF {
    unsafe fn init(&self) -> Result<(), &'static str> {
        self.inner.lock(|grf| grf.init());
        Ok(())
    }

    fn get_device_type(&self) -> DeviceType {
        DeviceType::GPIO
    }
}
