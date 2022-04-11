
use super::Driver;
use core::fmt;
use super::serial;

pub trait Console {

    // Tx
    fn write_char(&self, ch: char);
    fn write_str(&self, str: &str);
    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;

    fn flush(&self);
    fn write_drain(&self);

    // Rx
    fn read_char(&self) -> char;
    fn read_char_noblock(&self) -> Option<char>;

    fn read_drain(&self);
} 

pub trait ALL = Driver + Console;
/// Return a reference to the console.
pub fn console() -> &'static impl ALL  {
    &serial::UART2
}


//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {

    self::console().write_fmt(args).unwrap();
}

/// Prints without a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::drivers::console::_print(format_args!($($arg)*)));
}

/// Prints with a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::drivers::console::_print(format_args!($($arg)*));
        $crate::print!("\n");
    })
}

// panic handler

fn _panic_print(args: fmt::Arguments) {
    
    use crate::drivers::gpio;
    let panic_gpio = gpio::panic_gpio();
    let panic_uart = serial::panic_uart();

    unsafe {
        panic_gpio.map_uart2();
        panic_uart.init().unwrap();
        panic_uart.write_fmt(args).unwrap()
    }
}

/// Prints with a newline - only use from the panic handler.
#[macro_export]
macro_rules! panic_println {
    ($($arg:tt)*) => ({
        _panic_print(format_args_nl!($($arg)*));
    })
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut el: u64 = 0;
    unsafe{ asm!(
        "MRS {0}, CurrentEL",
        out(reg) el,
    )};
    if let Some(args) = info.message() {
        panic_println!("\ncurrent el{}\nKernel panic: {}", el>>2, args);
    } else {
        panic_println!("\ncurrent el{}\nKernel panic!", el>>2);
    }

    loop {
        cortex_a::asm::wfe();
    }
}
