

/// The board's physical memory map.
#[rustfmt::skip]
#[allow(dead_code)]

pub const START: usize = 0xF800_0000;
pub const END_INCLUSIVE: usize = 0xFFFF_FFFF;

pub const BOARD_DEFAULT_LOAD_ADDRESS:   usize = 0x0008_0000;
/// Physical devices.
pub const GRF_BASE:			        usize =	0xFF77_0000;
pub const UART2_BASE:		        usize =	0xFF1A_0000;
pub const TIMER0_5_BASE:            usize = 0xFF85_0000;
pub const TIMER6_11_BASE:           usize = 0xFF85_8000;
pub const GICD_BASE:                usize = 0xFEE0_0000;
pub const GICR_BASE:                usize = 0xFEF0_0000;


/// The address on which the firmware loads every binary by default.
#[inline(always)]
#[allow(dead_code)]
pub fn board_default_load_addr() -> *const u64 {
    self::BOARD_DEFAULT_LOAD_ADDRESS as _
}
