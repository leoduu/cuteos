// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2021 Andre Richter <andre.o.richter@gmail.com>

//! Architectural synchronous and asynchronous exception handling.
//!
//! # Orientation
//!
//! Since arch modules are imported into generic modules using the path attribute, the path of this
//! file is:
//!
//! crate::exception::arch_exception

use core::{cell::UnsafeCell, fmt};
use cortex_a::{asm::barrier, registers::*};
use tock_registers::{
    interfaces::{Readable, Writeable},
    registers::InMemoryRegister,
};
use crate::println;

pub mod asynchronous;

// Assembly counterpart to this file.
global_asm!(include_str!("exception.S"));

//--------------------------------------------------------------------------------------------------
// Private Definitions
//--------------------------------------------------------------------------------------------------

/// Wrapper structs for memory copies of registers.
#[repr(transparent)]
struct SpsrEL1(InMemoryRegister<u64, SPSR_EL1::Register>);
struct EsrEL1(InMemoryRegister<u64, ESR_EL1::Register>);

/// The exception context as it is stored on the stack on exception entry.
#[repr(C)]
pub struct ExceptionContext {
    /// General Purpose Registers.
    gpr: [u64; 30],

    /// The link register, aka x30.
    lr: u64,

    /// Exception link register. The program counter at the time the exception happened.
    elr_el1: u64,

    /// Saved program status.
    spsr_el1: SpsrEL1,

    // Exception syndrome register.
    esr_el1: EsrEL1,
}

//--------------------------------------------------------------------------------------------------
// Private Code
//--------------------------------------------------------------------------------------------------

/// Prints verbose information about the exception and then panics.
fn default_exception_handler(exc: &ExceptionContext) {
    panic!(
        "\n\nCPU Exception!\n\
        {}",
        exc
    );
}

//------------------------------------------------------------------------------
// Default 
//------------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn default_sync(e: &mut ExceptionContext) {
    println!("sync");
    default_exception_handler(e);
}

#[no_mangle]
unsafe extern "C" fn default_serror(e: &mut ExceptionContext) {
    println!("serror");
    default_exception_handler(e);
}

#[no_mangle]
unsafe extern "C" fn default_irq(e: &mut ExceptionContext) {
    println!("serror");
    default_exception_handler(e);
}

#[no_mangle]
unsafe extern "C" fn default_fiq(e: &mut ExceptionContext) {
    println!("fiq");
    default_exception_handler(e);
}

//------------------------------------------------------------------------------
// Current, EL0
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------
// Current, ELx
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------
// Lower, AArch64
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------
// Lower, AArch32
//------------------------------------------------------------------------------


//------------------------------------------------------------------------------
// Misc
//------------------------------------------------------------------------------

/// Human readable SPSR_EL1.
#[rustfmt::skip]
impl fmt::Display for SpsrEL1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Raw value.
        writeln!(f, "SPSR_EL1: {:#010x}", self.0.get())?;

        let to_flag_str = |x| -> _ {
            if x { "Set" } else { "Not set" }
         };

        writeln!(f, "      Flags:")?;
        writeln!(f, "            Negative (N): {}", to_flag_str(self.0.is_set(SPSR_EL1::N)))?;
        writeln!(f, "            Zero     (Z): {}", to_flag_str(self.0.is_set(SPSR_EL1::Z)))?;
        writeln!(f, "            Carry    (C): {}", to_flag_str(self.0.is_set(SPSR_EL1::C)))?;
        writeln!(f, "            Overflow (V): {}", to_flag_str(self.0.is_set(SPSR_EL1::V)))?;

        let to_mask_str = |x| -> _ {
            if x { "Masked" } else { "Unmasked" }
        };

        writeln!(f, "      Exception handling state:")?;
        writeln!(f, "            Debug  (D): {}", to_mask_str(self.0.is_set(SPSR_EL1::D)))?;
        writeln!(f, "            SError (A): {}", to_mask_str(self.0.is_set(SPSR_EL1::A)))?;
        writeln!(f, "            IRQ    (I): {}", to_mask_str(self.0.is_set(SPSR_EL1::I)))?;
        writeln!(f, "            FIQ    (F): {}", to_mask_str(self.0.is_set(SPSR_EL1::F)))?;

        write!(f, "      Illegal Execution State (IL): {}",
            to_flag_str(self.0.is_set(SPSR_EL1::IL))
        )
    }
}

impl EsrEL1 {
    #[inline(always)]
    fn exception_class(&self) -> Option<ESR_EL1::EC::Value> {
        self.0.read_as_enum(ESR_EL1::EC)
    }
}

/// Human readable ESR_EL1.
#[rustfmt::skip]
impl fmt::Display for EsrEL1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Raw print of whole register.
        writeln!(f, "ESR_EL1: {:#010x}", self.0.get())?;

        // Raw print of exception class.
        write!(f, "      Exception Class         (EC) : {:#x}", self.0.read(ESR_EL1::EC))?;

        // Exception class.
        let ec_translation = match self.exception_class() {
            Some(ESR_EL1::EC::Value::DataAbortCurrentEL) => "Data Abort, current EL",
            _ => "N/A",
        };
        writeln!(f, " - {}", ec_translation)?;

        // Raw print of instruction specific syndrome.
        write!(f, "      Instr Specific Syndrome (ISS): {:#x}", self.0.read(ESR_EL1::ISS))
    }
}

impl ExceptionContext {
    #[inline(always)]
    fn exception_class(&self) -> Option<ESR_EL1::EC::Value> {
        self.esr_el1.exception_class()
    }

    #[inline(always)]
    fn fault_address_valid(&self) -> bool {
        use ESR_EL1::EC::Value::*;

        match self.exception_class() {
            None => false,
            Some(ec) => matches!(
                ec,
                InstrAbortLowerEL
                    | InstrAbortCurrentEL
                    | PCAlignmentFault
                    | DataAbortLowerEL
                    | DataAbortCurrentEL
                    | WatchpointLowerEL
                    | WatchpointCurrentEL
            ),
        }
    }

    pub fn new_for_memory(elr_el1: u64, spsr_el1: u64) -> Self {
        ExceptionContext {
            gpr: [0x0; 30],
            lr: 0,
            elr_el1, 
            spsr_el1: SpsrEL1(InMemoryRegister::new(spsr_el1)),
            esr_el1: EsrEL1(InMemoryRegister::new(0)),
        }
    }
}

/// Human readable print of the exception context.
impl fmt::Display for ExceptionContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.esr_el1)?;

        if self.fault_address_valid() {
            writeln!(f, "FAR_EL1: {:#018x}", FAR_EL1.get() as usize)?;
        }

        writeln!(f, "{}", self.spsr_el1)?;
        writeln!(f, "ELR_EL1: {:#018x}", self.elr_el1)?;
        writeln!(f)?;
        writeln!(f, "General purpose register:")?;

        #[rustfmt::skip]
        let alternating = |x| -> _ {
            if x % 2 == 0 { "   " } else { "\n" }
        };

        // Print two registers per line.
        for (i, reg) in self.gpr.iter().enumerate() {
            write!(f, "      x{: <2}: {: >#018x}{}", i, reg, alternating(i))?;
        }
        write!(f, "      lr : {:#018x}", self.lr)
    }
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------
/// Kernel privilege levels.
#[allow(missing_docs)]
#[derive(PartialEq)]
pub enum PrivilegeLevel {
    User,
    Kernel,
    Hypervisor,
    Monitor,
    Unknown,
}

/// The processing element's current privilege level.
pub fn current_privilege_level() -> (PrivilegeLevel, &'static str) {
    let el = CurrentEL.read_as_enum(CurrentEL::EL);
    match el {
        Some(CurrentEL::EL::Value::EL3) => (PrivilegeLevel::Monitor, "EL3"),
        Some(CurrentEL::EL::Value::EL2) => (PrivilegeLevel::Hypervisor, "EL2"),
        Some(CurrentEL::EL::Value::EL1) => (PrivilegeLevel::Kernel, "EL1"),
        Some(CurrentEL::EL::Value::EL0) => (PrivilegeLevel::User, "EL0"),
        _ => (PrivilegeLevel::Unknown, "Unknown EL"),
    }
}

/// Init exception handling by setting the exception vector base address register.
///
/// # Safety
///
/// - Changes the HW state of the executing core.
/// - The vector table and the symbol `__exception_vector_table_start` from the linker script must
///   adhere to the alignment and size constraints demanded by the ARMv8-A Architecture Reference
///   Manual.
pub unsafe fn exception_init() {
    // Provided by exception.S.
    extern "Rust" {
        static __exception_vector: UnsafeCell<()>;
    }

    VBAR_EL1.set(__exception_vector.get() as u64);

    // Force VBAR update to complete before next instruction.
    barrier::isb(barrier::SY);
}
