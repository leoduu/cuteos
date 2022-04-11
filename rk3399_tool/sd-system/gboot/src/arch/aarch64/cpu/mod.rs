
use cortex_a::{asm, registers::*};
use tock_registers::interfaces::{Writeable, Readable};

use crate::println;

// Assembly counterpart to this file.
global_asm!(include_str!("cpu.S"));

extern "C" {
    pub fn lowlevel_init();
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------
#[inline(always)]
pub unsafe fn switch_el2_to_el1(kernel_point_entry: u64) -> ! {
    // Enable timer counter registers for EL1.
    // CNTHCTL_EL2.write(CNTHCTL_EL2::EL1PCEN::SET 
    //                 + CNTHCTL_EL2::EL1PCTEN::SET
    //                 + CNTHCTL_EL2::EL0PCEN::SET
    //                 + CNTHCTL_EL2::EL1PCTEN::SET);
    CNTHCTL_EL2.set(0x0603);

    // No offset for reading the counters.
    CNTVOFF_EL2.set(0);

    // Initialize the SCTLR_EL1 register before entering EL1.
    SCTLR_EL1.set(0x0);

    // Set EL1 execution state to AArch64.
    HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);

    // Set up a simulated exception return.
    //
    // First, fake a saved program status where all interrupts were masked and SP_EL1 was used as a
    // stack pointer.
    SPSR_EL2.write(
        SPSR_EL2::D::Unmasked
            + SPSR_EL2::A::Unmasked
            + SPSR_EL2::I::Unmasked
            + SPSR_EL2::F::Unmasked
            + SPSR_EL2::M::EL1h,
    );

    // Second, let the link register point to kernel_init().
    ELR_EL2.set(kernel_point_entry as *const () as u64);

    // Set up SP_EL1 (stack pointer), which will be used by EL1 once we "return" to it. Since there
    // are no plans to ever return to EL2, just re-use the same stack.
    SPSel.write(SPSel::SP::ELx);
    SP_EL1.set(0x2000000);

    asm::eret();
}


/// The Rust entry of the `kernel` binary.
///
/// The function is called from the assembly `_start` function.
#[no_mangle]
pub unsafe fn _start_rust() -> ! {

    lowlevel_init();

    crate::kernel_init()
}

/// Used by `arch` code to find the early boot core.
#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static BOOT_CORE_ID: u64 = 0;


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
