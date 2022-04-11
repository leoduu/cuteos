// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2021 Andre Richter <andre.o.richter@gmail.com>

//! Architectural asynchronous exception handling.
//!
//! # Orientation
//!
//! Since arch modules are imported into generic modules using the path attribute, the path of this
//! file is:
//!
//! crate::exception::asynchronous::arch_asynchronous

use cortex_a::registers::*;
use tock_registers::interfaces::Readable;

//--------------------------------------------------------------------------------------------------
// Private Definitions
//--------------------------------------------------------------------------------------------------

trait DaifField {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register>;
}

struct Debug;
struct SError;
struct IRQ;
struct FIQ;

//--------------------------------------------------------------------------------------------------
// Private Code
//--------------------------------------------------------------------------------------------------

impl DaifField for Debug {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register> {
        DAIF::D
    }
}

impl DaifField for SError {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register> {
        DAIF::A
    }
}

impl DaifField for IRQ {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register> {
        DAIF::I
    }
}

impl DaifField for FIQ {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register> {
        DAIF::F
    }
}

fn is_masked<T>() -> bool
where
    T: DaifField,
{
    DAIF.is_set(T::daif_field())
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

/// Print the AArch64 exceptions status.
#[rustfmt::skip]
pub fn print_state() {
    use crate::println;

    let to_mask_str = |x| -> _ {
        if x { "Masked" } else { "Unmasked" }
    };

    println!("      Debug:  {}", to_mask_str(is_masked::<Debug>()));
    println!("      SError: {}", to_mask_str(is_masked::<SError>()));
    println!("      IRQ:    {}", to_mask_str(is_masked::<IRQ>()));
    println!("      FIQ:    {}", to_mask_str(is_masked::<FIQ>()));
}
