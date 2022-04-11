
pub mod basic_timer;
pub mod system_timer;
//pub mod software_timer;

use crate::bsp::board;

pub fn system_timer() -> &'static impl system_timer::Interface {
    board::generic_timer()
}

// pub fn basic_timer() -> &'static impl basic_timer::Interface {

// }

// pub fn software_timer() -> &'static Mutex<impl SoftwareTimer> {
//     &software_timer::SOFTWARE_TIMER
// }

