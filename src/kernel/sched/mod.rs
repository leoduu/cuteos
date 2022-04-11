
mod round_robin;
use alloc::sync::Arc;
use lazy_static::*;
use crate::drivers::timer::{system_timer, system_timer::Interface};
use crate::sync::Mutex;
use crate::process::Process;

pub const MAX_PRIORITY: usize = 15;
const OS_TICK_PER_SECOND: u64 = 1000;     // secudler every 1ms

use round_robin::RRScheduler as KernelScheduler;

lazy_static! {
    pub static ref SCHEDULER: Mutex<KernelScheduler> = 
        Mutex::new(KernelScheduler::new());
}

pub trait Scheduler {
    fn run(&mut self) -> !;

    fn push(&mut self, process: Arc<Process>);
    fn pop(&mut self) -> Option<Arc<Process>>;
    
    fn schedule(&mut self);
}  

pub fn scheduler() -> &'static Mutex<KernelScheduler> {
    &*SCHEDULER
}

pub unsafe fn init() {
    let sys_timer = system_timer();
    let handler = || (SCHEDULER.lock(|sched| sched.schedule()));
    sys_timer.init().unwrap();
    sys_timer.set_irq_handler(handler);
}
