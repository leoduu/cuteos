
use crate::drivers::{DeviceType, Driver};
use core::time::Duration;


pub trait Interface {

    fn init(&self) -> Result<(), &'static str>;

    fn get_cycle(&self) -> Duration;

    fn set_irq_handler(&self, handler: fn());

    fn set_periodic(&self, duration: Duration);
}

struct SysTimer;

impl Driver for SysTimer {

    fn device_type(&self) -> DeviceType {
        DeviceType::SysTimer
    }

    fn compatible(&self) -> &'static str {
        "System Timer"
    }
}



