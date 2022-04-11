use crate::bsp::map;

pub mod timer;

pub static TIMER0: timer::Timer = unsafe {
    timer::Timer::new(map::physical::TIMER0_5_BASE)
};


pub fn timer(num: usize) -> timer::Timer {
    unsafe {
        match num {
            0..5 => timer::Timer::new(map::physical::TIMER0_5_BASE + 0x20 * num),
            6..11 => timer::Timer::new(map::physical::TIMER6_11_BASE + 0x20 * (num-6)),
            _ => panic!("error Timer num")
        }
    }
}
