use core::time::Duration;
use crate::bsp::mmio::MMIODerefWrapper;
use tock_registers::{
    interfaces::{Readable, ReadWriteable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};
use crate::drivers::timer::{BasicTimer, BasicTimerMode};
use crate::arch::interrupt;
use super::consts::*;

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
    intr: usize,
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
    pub const unsafe fn new(timer_num: usize) -> Self {
        let phy_addr: usize = match timer_num {
            0..5 => super::map::TIMER0_5_BASE + 0x20 * timer_num,
            6..11 => super::map::TIMER6_11_BASE + 0x20 * (timer_num-6),
            _ => panic!("error Timer num")
        };
        Self {
            intr: INTR::TIMER0 as usize + timer_num,
            regs: Registers::new(phy_addr),
        }
    }

    fn set_count(&self, us_init: u64, us_cmp: u64) {
        
        // period = (1/24000000)*1000000 = 1/24us
        let cnt01 = us_cmp * 24;
        let cnt23 = us_init * 24;
        // initial and reload value
        self.regs.LOAD_COUNT2.set((cnt23 & 0xFFFF_FFFF) as u32);
        self.regs.LOAD_COUNT3.set(((cnt23 >> 32) & 0xFFFF_FFFF) as u32);
        // compare value
        self.regs.LOAD_COUNT0.set((cnt01 & 0xFFFF_FFFF) as u32);
        self.regs.LOAD_COUNT1.set(((cnt01 >> 32) & 0xFFFF_FFFF) as u32);
    }

    #[inline(always)]
    fn current_value(&self) -> u64 {
        (self.regs.CURRENT_VALUE1.get() as u64) << 32 | self.regs.CURRENT_VALUE0.get() as u64
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

}

impl BasicTimer for TimerInner {

    fn init(&self, duration: Duration, mode: BasicTimerMode) {
        
        self.stop();

        match mode {
            BasicTimerMode::Count | BasicTimerMode::IRQOnce | BasicTimerMode::Spin => 
                self.regs.CONTROL_REG.modify(CONTROL_REG::TIMER_MODE::USER_DEFINED_COUNT),
            BasicTimerMode::IRQPeriodic => 
                self.regs.CONTROL_REG.modify(CONTROL_REG::TIMER_MODE::FREE_RUNNING),
            _ => panic!("error timer cnt mode")
        }

        match mode {
            BasicTimerMode::Count | BasicTimerMode::Spin => 
                self.regs.CONTROL_REG.modify(CONTROL_REG::INT_EN::MASK),
            BasicTimerMode::IRQOnce | BasicTimerMode::IRQPeriodic => {
                self.regs.CONTROL_REG.modify(CONTROL_REG::INT_EN::NOT_MASK);

                interrupt::irq_handler_enable(self.intr);
            },
            _ => panic!("error timer irq mode")
        }

        self.set_count(0, duration.as_micros() as u64);
    }

    #[inline(always)]
    fn set_irq_handler(&self, handler: fn()) {
        unsafe {
            interrupt::irq_install_handler(self.intr, handler);
        }
    }

    #[inline(always)]
    fn clear_irq(&self) {
        self.regs.INT_STATUS.write(INT_STATUS::INT_PD::SET);
    }

    fn delay(&self, duration: Duration) {

        self.regs.CONTROL_REG.write(CONTROL_REG::TIMER_MODE::USER_DEFINED_COUNT
                                    + CONTROL_REG::INT_EN::MASK
                                    + CONTROL_REG::TIMER_EN::DISABLE);
        
        self.set_count(0, duration.as_micros() as u64);
        self.start();

        while self.regs.INT_STATUS.matches_all(INT_STATUS::INT_PD::CLEAR) {}
        self.regs.INT_STATUS.set(1);   
        self.stop();
    }

    #[inline(always)]
    fn start(&self) {
        self.regs.CONTROL_REG.modify(CONTROL_REG::TIMER_EN::ENABLE);
    }

    #[inline(always)]
    fn stop(&self) {
        self.regs.CONTROL_REG.modify(CONTROL_REG::TIMER_EN::DISABLE);
    }

    #[inline(always)]
    fn get_cycle(&self) -> Duration {
        Duration::from_micros(self.current_value() / 24)
    }
}
