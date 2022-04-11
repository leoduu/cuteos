use super::{Driver, gpio, serial};

struct DriverManager {
    drivers: [&'static (dyn Driver + Sync); 2],
}

//--------------------------------------------------------------------------------------------------
// Global instances
//--------------------------------------------------------------------------------------------------

static BSP_DRIVER_MANAGER: DriverManager = DriverManager {
    drivers: [
        &serial::UART2,
        &gpio::GPIOGRF,
    ],
};

impl DriverManager {
    fn post_device_driver_init(&self) {
        // Configure UART2's output pins.
        gpio::GPIOGRF.map_uart2();
    }

    fn all_device_drivers(&self) -> &[&'static (dyn Driver + Sync)] {
        &self.drivers[..]
    }
}

pub unsafe fn driver_init() {

    let dm = &BSP_DRIVER_MANAGER;
    dm.post_device_driver_init();

    for i in dm.all_device_drivers().iter() {

        if let Err(e) = i.init() {
            panic!("{}", e);
        }
        cortex_a::asm::nop();
    }
}