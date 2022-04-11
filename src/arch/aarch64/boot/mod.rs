

// Assembly counterpart to this file.
//global_asm!(include_str!("boot.S"));
global_asm!(include_str!("boot.S"));

extern "C" {
    pub fn warm_reset() -> !;
    pub fn get_RVBAR_EL3() -> usize;
    pub fn lowlevel_init();
    pub fn cpu_interface_init();
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

use cortex_a::registers::*;
use tock_registers::interfaces::{Readable, Writeable};

/// Used by `arch` code to find the early boot core.
#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static BOOT_CORE_ID: u64 = 0;

/// Prepares the transition from EL2 to EL1.
///
/// # Safety
///
/// - The `bss` section is not initialized yet. The code must not use or reference it in any way.
/// - The HW state of EL1 must be prepared in a sound way.
#[inline(always)]
unsafe fn prepare_el2_to_el1_transition(_phys_stack_end_addr: u64) {
    // Enable timer counter registers for EL1.
    CNTHCTL_EL2.write(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);

    // No offset for reading the counters.
    CNTVOFF_EL2.set(0);

    // Set EL1 execution state to AArch64.
    HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);

    // Set up a simulated exception return.
    //
    // First, fake a saved program status where all interrupts were masked and SP_EL1 was used as a
    // stack pointer.
    SPSR_EL2.write(
        SPSR_EL2::D::Masked
            + SPSR_EL2::A::Masked
            + SPSR_EL2::I::Masked
            + SPSR_EL2::F::Masked
            + SPSR_EL2::M::EL1h,
    );

    // Second, let the link register point to kernel_init().
    ELR_EL2.set(crate::kernel::kernel_init as *const () as u64);

    // Set up SP_EL1 (stack pointer), which will be used by EL1 once we "return" to it. Since there
    // are no plans to ever return to EL2, just re-use the same stack.
    SP_EL1.set(_phys_stack_end_addr);
}

/// The Rust entry of the `kernel` binary.
///
/// The function is called from the assembly `_start` function.
#[no_mangle]
pub unsafe fn _start_rust(_phys_stack_end_addr: u64) -> ! {

    prepare_el2_to_el1_transition(_phys_stack_end_addr);

    cortex_a::asm::eret();
}


pub fn get_aarch() -> &'static str { 
    if core::mem::size_of::<usize>() == 8 {
        "Aarch64"
    } else {
        "AArch32"
    }
}

pub fn current_privilege_level_str() -> &'static str {
    let el = CurrentEL.read(CurrentEL::EL);
    match el {
        3 => "EL3",
        2 => "EL2",
        1 => "EL1",
        0 => "EL0",
        _ => "Unknown",
    }
}

pub fn current_privilege_level() -> usize{
    CurrentEL.read(CurrentEL::EL) as usize
}

