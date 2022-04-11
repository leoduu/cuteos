
use alloc::collections::VecDeque;

use crate::arch::context::{self, os_cpu_switch};
use crate::drivers::console::{console, Interface};
use crate::drivers::timer::system_timer;
use crate::{println, print, info};
use crate::process::{Process, ProcessStatus};
use core::fmt;
use core::time::Duration;
use super::{Scheduler, MAX_PRIORITY, OS_TICK_PER_SECOND};
use alloc::sync::Arc;


pub struct RRScheduler {
    ready_list: VecDeque<Arc<Process>>,
    current: Option<Arc<Process>>,
    os_tick: u128,
}
unsafe impl Sync for RRScheduler {}
unsafe impl Send for RRScheduler {}

impl RRScheduler {

    pub fn new() -> Self {
        RRScheduler {
            ready_list: VecDeque::new(),
            current: None,
            os_tick: 0,
        }
    }

    pub fn find_by_name(&self, _name: &str) -> Option<&Process> {

        return None;
    }
    
    pub fn find(&self, _pid: usize) -> Option<&Process> {

        return None;
    }
}

impl Scheduler for RRScheduler {
    fn run(&mut self) -> ! {

        if let Some(process) = self.ready_list.pop_front() {
            
            let mut pcb = process.context_access();
            let context = &mut pcb.context;

            unsafe {
                context::os_cpu_switch_to(context);
            }
        } else {
            panic!("scheduler run error")
        }

    }

    fn push(&mut self, process: Arc<Process>) {
        self.ready_list.push_back(process);
    }

    fn pop(&mut self) -> Option<Arc<Process>> {
        self.ready_list.pop_front()
    }

    fn schedule(&mut self) {
            
    }
}

impl fmt::Display for RRScheduler {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        writeln!(f, "Round Robin Scheduler")?;
        
        for p in &self.ready_list {
            p.show_name();
            write!(f, " ")?;
        }
        writeln!(f)
    }  
}
