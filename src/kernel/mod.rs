
use crate::{println, info};

#[no_mangle]
pub unsafe fn kernel_init() -> ! {
    use crate::drivers;
    use crate::arch::boot::*;
    use crate::arch;
    use crate::drivers::console::{console, Interface};
    use crate::mem;

    // if let Err(string) = mmu::mmu().enable_mmu_and_caching() {
    //     panic!("MMU: {}", string);
    // }

    arch::exception::exception_init();
    // arch::interrupt::enable_interrupt();
    drivers::drivers_init();    
    mem::heap_init();

    info!(
        "{} {} {} version {}",
        env!("CARGO_PKG_NAME"),
        get_aarch(),
        current_privilege_level_str(),
        env!("CARGO_PKG_VERSION")
    );


    // info!("MMU online. Special regions:");
    // crate::bsp::board::mmu::virt_mem_layout().print_layout();

    info!("Drivers loaded:");
    drivers::drivers_list_print();

    info!("Exception handling state:");
    arch::exception::asynchronous::print_state();


    println!("{:?}", crate::mem::allocator());

    info!("switch to first process (EL0) ...\n");
    console().flush();
    
    kernel_main();
}



fn kernel_main() -> ! {
    // use crate::sched::{self, Scheduler, scheduler};
    use crate::shell;

    unsafe {
        // sched::init();
        shell::shell_init();

        shell::shell_entry();
        
        //scheduler().lock(|sched| sched.run())
    }
}

unsafe fn ldaxr_test() {

    //let data: usize = 0x2093950;
    let data: usize = 0x2080000;
    let mut output: usize = 0;

    asm!(
        "mov x0, {0}",
        "ldaxr {1}, [x0]",
        in(reg) data,
        out(reg) output,
    );

    println!("output {:#x}", output);
}


