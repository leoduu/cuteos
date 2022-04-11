

global_asm!(include_str!("context.S"));
extern "C" {
    pub fn os_cpu_switch(context: &mut StackFrame);
    pub fn os_cpu_switch_to(context: &mut StackFrame) -> !;
}

#[no_mangle]
static mut KERNEL_SP: usize = 0;

#[repr(C)]
pub struct StackFrame {
    gpr: [usize; 31],
    sp: usize,
    elr: usize,
    spsr: usize,
    _reserved: usize,
}

impl StackFrame {
    pub const fn new(elr: usize, sp: usize) -> Self {

        Self {
            gpr: [0; 31],
            sp,
            elr,
            spsr: 0,
            _reserved: 0,
        }
    }
}


#[no_mangle] 
pub fn printx0(x0: usize) -> usize {
    use crate::println;
    use crate::drivers::console::{console, Interface};

    println!("x0 {:#x}", x0);
    console().flush();
    x0
}

#[no_mangle]
pub unsafe fn memory_dump(addr: *const u8, size: usize) {

    use crate::{print, println};
    use crate::drivers::console::{console, Interface};
    
    for i in 0..size {
        if i % 8 == 0 {
            println!();
            print!("{:?} ", addr.offset(i as isize));
        }
        let data = addr.offset(i as isize).read_volatile();
        if data < 0x10 {
            print!("0");
        } 
        
        print!("{:x} ", data);
    }
    println!();
    console().flush();
}
