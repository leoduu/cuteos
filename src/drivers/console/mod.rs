
use core::fmt;
use crate::bsp::board::console;

//pub static CONSOLE: Mutex<Option<dyn Interface>> = Mutex::new(None);

pub trait Interface {

    fn write_char(&self, ch: char);
    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
    
    fn read_char(&self) -> char;

    fn flush(&self);
} 


/// Return a reference to the console.
pub fn console() -> &'static impl Interface {
    console::console()
}


