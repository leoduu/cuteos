
use core::fmt;
use crate::drivers::console;
use crate::bsp::board;


#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use console::{console, Interface};
    console().write_fmt(args).unwrap();
}

/// Prints without a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::log::_print(format_args!($($arg)*)));
}

/// Prints with a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::log::_print(format_args_nl!($($arg)*));
    })
}


/// Prints an info, with a newline.
#[macro_export]
macro_rules! info {
    ($string:expr) => ({
        use crate::drivers::timer::{system_timer, system_timer::Interface};

        let timestamp = system_timer().get_cycle();
        let timestamp_subsec_us = timestamp.subsec_micros();

        $crate::log::_print(format_args_nl!(
            concat!("[  {:>3}.{:06}] ", $string),
            timestamp.as_secs(),
            timestamp_subsec_us % 1_000_000,
        ));
    });
    ($format_string:expr, $($arg:tt)*) => ({
        use crate::drivers::timer::{system_timer, system_timer::Interface};

        let timestamp = system_timer().get_cycle();
        let timestamp_subsec_us = timestamp.subsec_micros();

        $crate::log::_print(format_args_nl!(
            concat!("[  {:>3}.{:06}] ", $format_string),
            timestamp.as_secs(),
            timestamp_subsec_us % 1_000_000,
            $($arg)*
        ));
    })
}

/// Prints a warning, with a newline.
#[macro_export]
macro_rules! warn {
    ($string:expr) => ({
        use crate::drivers::timer::{system_timer, system_timer::Interface};

        let timestamp = system_timer().get_cycle();
        let timestamp_subsec_us = timestamp.subsec_micros();

        $crate::log::_print(format_args_nl!(
            concat!("[W {:>3}.{:03}{:03}] ", $string),
            timestamp.as_secs(),
            timestamp_subsec_us / 1_000,
            timestamp_subsec_us % 1_000
        ));
    });
    ($format_string:expr, $($arg:tt)*) => ({
        use crate::drivers::timer::system_timer;

        let timestamp = system_timer().get_cycle();
        let timestamp_subsec_us = timestamp.subsec_micros();

        $crate::log::_print(format_args_nl!(
            concat!("[W {:>3}.{:03}{:03}] ", $format_string),
            timestamp.as_secs(),
            timestamp_subsec_us / 1_000,
            timestamp_subsec_us % 1_000,
            $($arg)*
        ));
    })
}


// panic handler
fn _panic_print(args: fmt::Arguments) {
    
    use fmt::Write;

    unsafe {
        board::console::panic_console_out().write_fmt(args).unwrap();
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

    use console::{console, Interface};
    
    console().flush();

    //unsafe {crate::arch::interrupt::disable_interrupt();}
    
    if let Some(args) = info.message() {
        panic_println!("loc {:?}\nKernel panic: {}",info.location(), args);
    } else {
        panic_println!("loc {:?}\nKernel panic!", info.location());
    }

    loop {
        cortex_a::asm::wfe();
    }
}
