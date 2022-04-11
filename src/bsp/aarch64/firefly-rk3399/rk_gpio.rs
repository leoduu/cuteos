
use crate::bsp::mmio::MMIODerefWrapper;

use tock_registers::{
    interfaces::{ReadWriteable},// ,Writeable},
    register_bitfields, register_structs,
    registers::ReadWrite,
};

register_bitfields! {
    u32,

    GPIO4B_IOMUX [
        WRITE_EN OFFSET(2) NUMBITS(16) [],

        SEL_5 OFFSET(10) NUMBITS(2) [],

        SEL_4 OFFSET(8) NUMBITS(2) [],

        SEL_3 OFFSET(6) NUMBITS(2) [],

        SEL_2 OFFSET(4) NUMBITS(2) [],

        SEL_1 OFFSET(2) NUMBITS(2) [
            reserved        = 3,
            uart2dbga_sout  = 2,
            sdmmc_data1     = 1,
            gpio            = 0
        ],

        SEL_0 OFFSET(0) NUMBITS(2) [
            reserved        = 3,
            uart2dbga_sin   = 2,
            sdmmc_data0     = 1,
            gpio            = 0
        ],
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    GPIORegisterBlock {
        (0x00000 => _reserved0),
        (0x0E02C => GPIO4B_IOMUX: ReadWrite<u32, GPIO4B_IOMUX::Register>),
        (0x0F7A4 => @END),
    }
}

/// Abstraction for the associated MMIO registers.
type Registers = MMIODerefWrapper<GPIORegisterBlock>;

pub struct GPIOGRFInner {
    regs: Registers,
}

impl GPIOGRFInner {
    /// Create an instance
    ///
    /// # Safety
    /// 
    /// prvide start address
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            regs: Registers::new(mmio_start_addr),
        }
    }

    pub fn init(&self) {
        
    }

    /// Set GPIO4B0 and GPIOB1 for uart2 
    pub fn map_uart2(&mut self) {
        self.regs
            .GPIO4B_IOMUX
            .modify(GPIO4B_IOMUX::SEL_0::uart2dbga_sin 
                    + GPIO4B_IOMUX::SEL_1::uart2dbga_sout
                    + GPIO4B_IOMUX::WRITE_EN.val(0b1100));
    }

    #[allow(dead_code)]
    pub fn compatible(&self) -> &'static str {
        "RK3399 GPIO"
    }
}

