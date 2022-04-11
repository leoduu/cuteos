use crate::bsp::map;

pub mod uart;


pub static UART2: uart::Uart = unsafe {
    uart::Uart::new(map::physical::UART2_BASE)
};

pub fn panic_uart() -> uart::Uart {
    unsafe {
        uart::Uart::new(map::physical::UART2_BASE)
    }
}
