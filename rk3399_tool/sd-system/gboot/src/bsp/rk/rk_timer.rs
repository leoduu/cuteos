use core::fmt;
use crate::{bsp::mmio::MMIODerefWrapper, println};
use tock_registers::{
    interfaces::{Readable, ReadWriteable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

//--------------------------------------------------------------------------------------------------
// Private Definitions
//--------------------------------------------------------------------------------------------------

//  UART registers
register_bitfields! {
    u32,

    // Timer Interrupt Stauts Register
    INT_STATUS  [
        INT_PD OFFSET(0) NUMBITS(1) [],
    ],

    // Timer Control Register
    CONTROL_REG [
        // Timer interrupt mask
        INT_EN      OFFSET(2) NUMBITS(1) [
            NOT_MASK = 0b1,
            MASK     = 0b0,
        ],
        // Timer mode
        TIMER_MODE  OFFSET(1) NUMBITS(1) [
            USER_DEFINED_COUNT = 0b1,
            FREE_RUNNING       = 0b0,
        ],
        // Timer enable
        TIMER_EN    OFFSET(0) NUMBITS(1) [
            ENABLE  = 0b1,
            DISABLE = 0b0,
        ],
    ],
}

register_structs! {
#[allow(non_snake_case)]
    RegisterBlock {
        (0x0000 => LOAD_COUNT0: ReadWrite<u32>),
        (0x0004 => LOAD_COUNT1: ReadWrite<u32>),
        (0x0008 => CURRENT_VALUE0: ReadOnly<u32>),
        (0x000C => CURRENT_VALUE1: ReadOnly<u32>),
        (0x0010 => LOAD_COUNT2: ReadWrite<u32>),
        (0x0014 => LOAD_COUNT3: ReadWrite<u32>),
        (0x0018 => INT_STATUS: ReadWrite<u32, INT_STATUS::Register>),
        (0x001C => CONTROL_REG: ReadWrite<u32, CONTROL_REG::Register>),
        (0x0020 => @END),
    }
}

/// Abstraction for the associated MMIO registers.
type Registers = MMIODerefWrapper<RegisterBlock>;

//--------------------------------------------------------------------------------------------------
// Public Definitions
//--------------------------------------------------------------------------------------------------
pub struct TimerInner {
    regs: Registers,
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

#[allow(dead_code)]
impl TimerInner {
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
        // 
        self.regs.CONTROL_REG.write(CONTROL_REG::TIMER_EN::DISABLE
                                        + CONTROL_REG::TIMER_MODE::USER_DEFINED_COUNT
                                        + CONTROL_REG::INT_EN::MASK);
        
        self.regs.LOAD_COUNT0.set(0);
        self.regs.LOAD_COUNT1.set(0);
    }

    pub fn delay_us(&self, cnt: u64) {

        self.regs.CONTROL_REG.write(CONTROL_REG::TIMER_EN::DISABLE
                                        + CONTROL_REG::TIMER_MODE::USER_DEFINED_COUNT
                                        + CONTROL_REG::INT_EN::MASK);
        
        // period = (1/24000000)*1000000 = 1/24us

        // initial value
        self.regs.LOAD_COUNT2.set(0);
        self.regs.LOAD_COUNT3.set(0);
        // compare value
        let cnt = cnt * 24;
        self.regs.LOAD_COUNT0.set((cnt & 0xFFFF_FFFF) as u32);
        self.regs.LOAD_COUNT1.set(((cnt >> 32) & 0xFFFF_FFFF) as u32);

        self.regs.CONTROL_REG.modify(CONTROL_REG::TIMER_EN::ENABLE);
        while self.regs.INT_STATUS.matches_all(INT_STATUS::INT_PD::CLEAR) {}
        self.regs.INT_STATUS.set(1);   
        self.regs.CONTROL_REG.modify(CONTROL_REG::TIMER_EN::DISABLE);
    }

    pub fn irq_delay_us(&self, cnt:u64) {
        self.clear_irq();
        self.regs.CONTROL_REG.write(CONTROL_REG::TIMER_EN::DISABLE
                                    + CONTROL_REG::TIMER_MODE::FREE_RUNNING
                                    + CONTROL_REG::INT_EN::NOT_MASK);

        // period = (1/24000000)*1000000 = 1/24us
        let cnt = cnt * 24;
        // initial value
        self.regs.LOAD_COUNT2.set(0);
        self.regs.LOAD_COUNT3.set(0);
        // compare value
        self.regs.LOAD_COUNT0.set((cnt & 0xFFFF_FFFF) as u32);
        self.regs.LOAD_COUNT1.set(((cnt >> 32) & 0xFFFF_FFFF) as u32);
        self.regs.CONTROL_REG.modify(CONTROL_REG::TIMER_EN::ENABLE);
    }

    #[inline(always)]
    pub fn current_val(&self) -> u64 {
        (self.regs.CURRENT_VALUE1.get() as u64) << 32 | self.regs.CURRENT_VALUE0.get() as u64
    }

    #[inline]
    pub fn clear_irq(&self) {
        self.regs.INT_STATUS.write(INT_STATUS::INT_PD::SET);
    }
    

    pub fn compatible(&self) -> &'static str {
        "RK3399 Timer"
    }

}

