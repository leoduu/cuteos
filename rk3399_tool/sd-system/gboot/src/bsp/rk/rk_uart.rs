use core::fmt;
use crate::bsp::mmio::MMIODerefWrapper;
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

    // Interrupt Enable Register
    IER [
        PROG_THRE_INT               7,
        MODEM_STATUS_INT            3,
        RECE_LINE_STATUS_INT        2,
        TRANS_HOLD_EMPTY_INT        1,
        RECE_DATA_AVAILABLE_INT     0,
    ],

    // FIFO Control Register
    FCR [
        RCVR_TRIGGER            OFFSET(6) NUMBITS(2) [
            TWO_LESS_FIFO = 0b11,
            HALF_FIFO     = 0b10,
            QUARTER_FIFO  = 0b01,
            ONE           = 0b00,
        ],
        TX_TRIGGER              OFFSET(4) NUMBITS(2) [
            ONE_FOUR_FIFO = 0b10,
            HALF_FIFO     = 0b11,
            TWO_IN_FIFO   = 0b01,
            EMPTY         = 0b00,
        ],
        DMA_MODE                OFFSET(3) NUMBITS(1) [],
        XMIT_FIFO_RESET         OFFSET(2) NUMBITS(1) [],
        RCVR_FIFO_RESET         OFFSET(1) NUMBITS(1) [],
        FIFO_ENABLE             OFFSET(0) NUMBITS(2) [],
    ],

    // Line Control Register
    LCR [
        DIV_LAT_ACCESS          OFFSET(7) NUMBITS(1) [],
        BREAK_CTRL              OFFSET(6) NUMBITS(1) [],
        EVEN_PARITY_SEL         OFFSET(4) NUMBITS(1) [],
        PARITY_SEL              OFFSET(3) NUMBITS(1) [],

        STOP_BITS_NUM           OFFSET(2) NUMBITS(1) [
            Bit_1_5 = 1,
            Bit_1   = 0
        ],
        DATA_LENGTH_SEL         OFFSET(0) NUMBITS(2) [
            Bit_8 = 0b11,
            Bit_7 = 0b10,
            Bit_6 = 0b01,
            Bit_5 = 0b00
        ],
    ],

    // Modem Control Register
    MCR [
        ALL 					OFFSET(0) NUMBITS(8) [],
    ],

    LSR [
        RECEIVE_FIFO_ERROR      OFFSET(7) NUMBITS(1) [],
        TRASNS_EMPTY            OFFSET(6) NUMBITS(1) [],
        TRANS_HOLD_REG_EMPTY    OFFSET(5) NUMBITS(1) [],
        BREAK_INT               OFFSET(4) NUMBITS(1) [],
        FRAMING_ERROR           OFFSET(3) NUMBITS(1) [],
        PARITY_ERROR            OFFSET(2) NUMBITS(1) [],
        OVERRUN_ERROR           OFFSET(1) NUMBITS(1) [],
        DATA_READY              OFFSET(0) NUMBITS(1) [],
    ],

    // Uart Status Register
    USR [
        RECEIVE_FIFO_FULL       OFFSET(4) NUMBITS(1) [
            Full      = 1,
            Not_full  = 0,
        ],
        RECEIVE_FIFO_NOT_EMPTY  OFFSET(3) NUMBITS(1) [
            Not_empty = 1,
            Empty     = 0,
        ],
        TRANS_FIFO_EMPTY        OFFSET(2) NUMBITS(1) [
            Empty     = 1,
            Not_empty = 0,
        ],
        TRANS_FIFO_NOT_FULL     OFFSET(1) NUMBITS(1) [
            Not_full  = 1,
            Full      = 0,
        ],
        UART_BUSY               OFFSET(0) NUMBITS(1) [
            Busy = 1,
            Idle = 0
        ],
    ],

    // Softer Reset Register
    SRR [ 
        XMIT_FIFO_RESET 		2,
        RCVR_FIFO_RESET 		1,
        UART_RESET 				0,
    ],

    // Shadow FIFO Enable
    // This is a shadow register for the FIFO enable bit (FCR[0])
    SFE [
        SHAOW_FIFO_ENABLE 		0,
    ],

    // Shadow RCVR Trigger
    // This is a shadow register for the RCVR trigger bits (FCR[7:6])
    SRT [
        SHADOW_RCVR_TRIGGER 	OFFSET(0) NUMBITS(2) [
            ONE           = 0b00,
            QUARTER_FIFO  = 0b01,
            HALF_FIFO     = 0b10,
            TWO_LESS_FIFO = 0b11,
        ],
    ],

    // Shadow TX Empty Trigger
    // This is a shadow register for the TX empty trigger bits (FCR[5:4])
    STET [
        SHADOW_TX_EMPTY_TRIGGER  OFFSET(0) NUMBITS(2) [
            EMPTY         = 0b00,
            TWO_IN_FIFO   = 0b01,
            ONE_FOUR_FIFO = 0b10,
            HALF_FIFO     = 0b11,
        ],
    ]
}

register_structs! {
#[allow(non_snake_case)]
    RegisterBlock {
        (0x0000 => RBP_THR_DLL: ReadWrite<u32>),
        (0x0004 => DLH_IER: ReadWrite<u32, IER::Register>),
        (0x0008 => IIR_FCR: ReadWrite<u32, FCR::Register>),
        (0x000C => LCR: ReadWrite<u32, LCR::Register>),
        (0x0010 => MCR: ReadWrite<u32, MCR::Register>),
        (0x0014 => LSR: ReadWrite<u32, LSR::Register>),
        (0x0018 => MSR: ReadWrite<u32>),
        (0x001C => SCR: ReadWrite<u32>),
        (0x0020 => _reserved0),
        (0x0030 => SRBR: ReadWrite<u32>),
        (0x0034 => _reserved1),
        (0x006C => STHR: ReadWrite<u32>),
        (0x0070 => FAR: ReadWrite<u32>),
        (0x0074 => TFR: ReadWrite<u32>),
        (0x0078 => RFW: ReadWrite<u32>),
        (0x007C => USR: ReadOnly<u32, USR::Register>),
        (0x0080 => TFL: ReadWrite<u32>),
        (0x0084 => RFL: ReadWrite<u32>),
        (0x0088 => SRR: WriteOnly<u32, SRR::Register>),
        (0x008C => SRTS: ReadWrite<u32>),
        (0x0090 => SBCR: ReadWrite<u32>),
        (0x0094 => SDMAM: ReadWrite<u32>),
        (0x0098 => SFE: ReadWrite<u32, SFE::Register>),
        (0x009C => SRT: ReadWrite<u32, SRT::Register>),
        (0x00A0 => STET: ReadWrite<u32, STET::Register>),
        (0x00A8 => DMASA: ReadWrite<u32>),
        (0x00AC => _reserved2),
        (0x00F4 => UCV: ReadWrite<u32>),
        (0x00F8 => CPR: ReadWrite<u32>),
        (0x00FC => CTR: ReadWrite<u32>),
        (0x0100 => @END),
    }
}

/// Abstraction for the associated MMIO registers.
type Registers = MMIODerefWrapper<RegisterBlock>;

// recevice mode
#[derive(PartialEq)]
pub enum BlockingMode {
    Blocking,
    NonBlocking,
}

//--------------------------------------------------------------------------------------------------
// Public Definitions
//--------------------------------------------------------------------------------------------------
pub struct UartInner {
    regs: Registers,
    chars_written: usize,
    chars_read: usize,
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

impl UartInner {
    /// Create an instance
    ///
    /// # Safety
    /// 
    /// prvide start address
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            regs: Registers::new(mmio_start_addr),
            chars_written: 0,
            chars_read: 0,
        }
    }

    /// Set up baud rate and characteristics.
    ///
    /// default band rate is 1.5MHZ
    /// 
    /// divosr from 24M input clock
    /// 
    /// 24000000 / 16 / 1.5M = 1
    pub fn init(&mut self) {
        // uart, rx fifo & tx fifo reset
        self.regs.SRR.write(SRR::XMIT_FIFO_RESET::SET
                                + SRR::RCVR_FIFO_RESET::SET
                                + SRR::UART_RESET::SET);
    
        // interrupt disable
        self.regs.DLH_IER.set(0x00);

        // uart clear modem contorl
        self.regs.MCR.write(MCR::ALL::CLEAR);

        // uart line control
        // 8bits data length , disable parity, 1 stop bit
        self.regs.LCR.modify(LCR::PARITY_SEL::CLEAR
                                + LCR::DATA_LENGTH_SEL::Bit_8 
                                + LCR::STOP_BITS_NUM::Bit_1);

        // set baud rate
        let rate = 1; // 24MHz / 16 / 1.5M
        self.regs.LCR.modify(LCR::DIV_LAT_ACCESS::SET);
        self.regs.RBP_THR_DLL.set(rate & 0xFF);
        self.regs.DLH_IER.set((rate>>8) & 0xFF);
        self.regs.LCR.modify(LCR::DIV_LAT_ACCESS::CLEAR);

        // set fifo
        self.regs.SFE.write(SFE::SHAOW_FIFO_ENABLE::SET);
        self.regs.SRT.write(SRT::SHADOW_RCVR_TRIGGER::TWO_LESS_FIFO);
        self.regs.STET.write(STET::SHADOW_TX_EMPTY_TRIGGER::TWO_IN_FIFO);
    }

    /// Block execution until the last buffered character has been physically put on the TX wire.
    pub fn flush(&mut self) {
        // Spin until the busy bit is cleared.
        while self.regs.USR.matches_all(USR::UART_BUSY::Busy) {
            cortex_a::asm::nop();
        }
    }

    pub fn drain(&mut self) {
        while self.regs.USR.matches_all(USR::TRANS_FIFO_EMPTY::CLEAR) {
            cortex_a::asm::nop();
        }
    }

    // send a char
    pub fn write(&mut self, ch: char) {
        // spin ,waiting 
        while self.regs.USR.matches_all(USR::TRANS_FIFO_NOT_FULL::Full) {
            cortex_a::asm::nop();
        }
        self.regs.RBP_THR_DLL.set(ch as u32);
        self.chars_written += 1;
    }

    // receive a char
    pub fn read(&mut self, mode: BlockingMode) -> Option<char> {
        // no data if the Data Ready bit is set or receive fifo is empty
        while self.regs.USR.matches_all(USR::RECEIVE_FIFO_NOT_EMPTY::Empty) {
            // immediately return in non-blocking mode
            if mode == BlockingMode::NonBlocking {
                return None;
            }
            // wait until a char was received in blocking mode
            cortex_a::asm::nop();
        }

        // read a char
        let ret = self.regs.RBP_THR_DLL.get() as u8 as char; 
        self.chars_read += 1;

        Some(ret)
    }

    #[allow(dead_code)]
    pub fn compatible(&self) -> &'static str {
        "RK3399 GPIO"
    }

}

impl fmt::Write for UartInner {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write(c);
        }

        Ok(())
    }
}

