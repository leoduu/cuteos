
#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points
#![cfg_attr(debug_assertions, allow(dead_code, unused_assignments, unreachable_patterns, unused_unsafe, unused_macros))]
#![feature(asm)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![feature(default_alloc_error_handler)]
#![feature(allocator_api)]
#![feature(ptr_internals)]
#![feature(exclusive_range_pattern)]
#![feature(const_for)]
#![feature(const_impl_trait)]
#![feature(core_intrinsics)]

#[allow(unused_imports)]
#[macro_use]
extern crate alloc;

mod utilities;
mod mem;
mod sync;
mod drivers;
// mod sched;
mod kernel;
mod shell;
mod log;

// aarch64
#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64/mod.rs"]
pub mod arch;
#[cfg(target_arch = "aarch64")]
#[path = "bsp/aarch64/mod.rs"]
pub mod bsp;


#[cfg(target_arch = "riscv")]
#[path = "arch/riscv/mod.rs"]
pub mod arch;

