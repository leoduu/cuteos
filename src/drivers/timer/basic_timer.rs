
use core::time::Duration;

pub enum TimerMode {
    Spin,
    Count,
    IRQOnce,
    IRQPeriodic,
}

pub trait Interface {
    
    fn init(&self, duration: Duration, mode: TimerMode);

    fn spin(&self, duration: Duration);

    fn set_irq_handler(&self, handler: fn());

    fn clear_irq(&self);

    fn start(&self);

    fn stop(&self);

    fn get_cycle(&self) -> Duration;
}


