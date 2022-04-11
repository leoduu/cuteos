

#![allow(clippy::upper_case_acronyms)]
#![feature(asm)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![no_main]
#![no_std]
#![cfg_attr(unused_assignments, debug_assertions, allow(dead_code, unused_imports, unreachable_patterns, unused_unsafe, unused_macros))]
#![feature(exclusive_range_pattern)]

mod bsp;
mod drivers;
mod sync;
mod arch;

use cortex_a::registers::CurrentEL;
use drivers::{console::{Console, console}, driver_manager};
use arch::*;
use tock_registers::interfaces::Readable;


unsafe fn kernel_init() -> ! {

    interrupt::enable_interrupt();
    exception::handling_init();
    driver_manager::driver_init();

    let el = CurrentEL.read(CurrentEL::EL);
    println!("[ML] current el{}", el);
    println!("[ML] version 0.11");
    print!("[ML] Wait for size");
    console().write_drain();
    console().read_drain();
    console().flush();

    for _ in 0..6 {
        console().write_char(6 as char);
    }
    println!();

    // Read the binary's size.
    let mut size: u32 = u32::from(console().read_char() as u8);
    size |= u32::from(console().read_char() as u8) << 8;
    size |= u32::from(console().read_char() as u8) << 16;
    size |= u32::from(console().read_char() as u8) << 24;
    
    println!("[ML] OK! Received size {} KB", size);

    // let kernel_addr: *mut u8 = bsp::map::board_default_load_addr() as *mut u8;
    let kernel_addr: *mut u8 = 0x208_0000 as *mut u8;

    for i in 0..size {
        let data = console().read_char() as u8;
        core::ptr::write_volatile(kernel_addr.offset(i as isize), data);
    }

    println!("[ML] Loaded! Entry point address = {:?}\n", kernel_addr);
    console().flush();

    // Use black magic to create a function pointer.
    let kernel: fn() -> ! = core::mem::transmute(kernel_addr as u64 + 0x8000);

    // Jump to loaded kernel!
    //kernel()

    boot::switch_el2_to_el1(kernel_addr as u64 + 0x8000)
}
