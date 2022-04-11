
use core::borrow::BorrowMut;
use core::cell::UnsafeCell;
use core::fmt;
use crate::{print, container_of, container_of_mut};
use crate::arch::context::StackFrame;
use crate::utilities::intrusive_linkedlist::*;
use alloc::alloc::Global;
use core::alloc::{Layout, Allocator};
use core::cell::{RefCell, RefMut};
 

const PROCESS_NAME_LEN: usize = 10;
type ProcessNameArr = [u8; PROCESS_NAME_LEN];

pub enum ProcessStatus {
    Init,
    Ready,
    Sleep,
    Terminal
}

impl fmt::Display for ProcessStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Init => write!(f, "Init"),
            Self::Ready => write!(f, "Ready"),
            Self::Sleep => write!(f, "Suspended"),
            Self::Terminal => write!(f, "Terminal"),
            _ => write!(f, "Undefined"),
        }
    }
}

pub struct Process {
    name: ProcessNameArr,                   // Process Name
    //pid: u32,                               // Process ID
    
    pcb: RefCell<ProcessControlBlock>,
}


impl Process {
    pub unsafe fn new(name: &str, 
                        entry: fn() -> !,
                        stack_start: usize,
                        stack_size: usize) -> Self
    {
        let mut name_arr: ProcessNameArr = [0; PROCESS_NAME_LEN];
        for i in 0..name.len() {
            name_arr[i] = name.as_bytes()[i];
        }

        Process { 
            name: name_arr, 
            pcb: RefCell::new(ProcessControlBlock::new(entry, stack_start, stack_size))
        }
    }

    #[inline(always)]
    pub fn show_name(&self) {
        for i in 0..PROCESS_NAME_LEN {
            print!("{}", self.name[i] as char);
        }
    }

    pub fn context_access(&self) -> RefMut<'_, ProcessControlBlock> {
        self.pcb.borrow_mut()
    }
}



impl fmt::Display for Process {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        writeln!(f, "\n process({:?}) :", self as *const Self)?;
        write!(  f, "\tname : ")?;
        for i in 0..PROCESS_NAME_LEN {
            write!(f, "{}", self.name[i] as char)?;
        }
        // writeln!(f, "")?;
        // writeln!(f, "\tprio : {}", self.priority)?;
        // writeln!(f, "\tstate: {}", self.status)?;
        // writeln!(f, "\tstack: 0x{:x} - 0x{:x}", self.stack_start, self.stack_start+self.stack_size)?;
        // writeln!(f, "\tsize : {} Bytes (0x{:x})", self.stack_size, self.stack_size)?;
        
        // writeln!(f, "\tlist :\tthis {:?}", &self.list as *const Node)?;
        // writeln!(f, "\t\tprev {:?}", self.list.prev())?;
        // writeln!(f, "\t\tnext {:?}", self.list.next())
        Ok(())
    }
}


pub struct ProcessControlBlock {

    pub status: ProcessStatus,      // Process status
    pub context: StackFrame,
    pub stack_start: usize,         // Stack start address    
    pub stack_size: usize,          // Stack Size
}

impl ProcessControlBlock {

    pub fn new(entry: fn()->!, stack_start: usize, stack_size: usize) -> Self {
        Self {
            status: ProcessStatus::Init,
            context: StackFrame::new(entry as usize, stack_start + stack_size),
            stack_start,
            stack_size
        }
    } 
}
