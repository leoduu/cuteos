

mod serial;
mod gpio;
pub mod timer;

pub mod console;
pub mod driver_manager;

#[derive(Debug, Eq, PartialEq)]
pub enum DeviceType {
    GPIO,
    Serial,
    Timer,
    // Net,
    // Gpu,
    // Input,
    // Block,
    // Rtc,
    // Intc,
}

pub trait Driver {

    unsafe fn init(&self) -> Result<(), &'static str>;

    fn get_device_type(&self) -> DeviceType;
}


