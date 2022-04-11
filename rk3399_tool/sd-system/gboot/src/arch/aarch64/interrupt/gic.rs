
use cortex_a::asm::barrier;
use crate::{bsp::mmio::MMIODerefWrapper, println, arch::boot};
use tock_registers::{
    interfaces::{Readable, ReadWriteable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};


// When access is Non-secure, in a system that supports two Security states:
register_bitfields! {
    u32,

    GICD_CTLR [
        RWP             OFFSET(30)  NUMBITS(1) [],
        E1NWF           OFFSET(7)   NUMBITS(1) [
            // 0b1  A PE that is asleep can be picked for 1 of N interrupts as determined by IMPLEMENTATION DEFINED controls.
            // 0b0  A PE that is asleep cannot be picked for 1 of N interrupts.
        ],
        DS              OFFSET(6)   NUMBITS(1) [
            // 0b1  Non-secure accesses are permitted to access and modify registers that control Group 0 interrupts.
            // 0b0  Non-secure accesses are not permitted to access and modify registers that control Group 0 interrupts.
        ],
        ARE_NS          OFFSET(5)   NUMBITS(4) [
            // 0b1  Affinity routing enabled for Non-secure state.
            // 0b0  Affinity routing disabled for Non-secure state.
        ],
        ARE_S           OFFSET(4)   NUMBITS(4) [
            // 0b1  Affinity routing enabled for secure state.
            // 0b0  Affinity routing disabled for secure state.
        ],
        EnableGrp1S     OFFSET(2)   NUMBITS(1) [],
        EnableGrp1NS    OFFSET(1)   NUMBITS(1) [],
        EnableGrp0      OFFSET(0)   NUMBITS(1) [],
    ],
    
    GICD_TYPER [
        ITLinesNumber   OFFSET(0)   NUMBITS(5) [],
    ],



    GICR_WAKER [
        ChildrenAsleep  OFFSET(2)   NUMBITS(1) [
            // 0b1  All interfaces to the connected PE are quiescent.
            // 0b0  An interface to the connected PE might be active.
        ],
        ProcessorSleep  OFFSET(1)   NUMBITS(1) [
            // 0b0  This PE is not in, and is not entering, a low power state.
            // 0b1  The PE is either in, or is in the process of entering, a low power state.
        ],
    ]
    
}

// RedistributorRegisterBlock
register_bitfields! {
    u64,
    
    GICR_TYPER [
        Affinity_Value  OFFSET(31)  NUMBITS(32) [],
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub Distributor_Register_Block {
        (0x0000 => CTLR: ReadWrite<u32, GICD_CTLR::Register>),
        (0x0004 => TYPER: ReadOnly<u32, GICD_TYPER::Register>),
        (0x0008 => IIDR: ReadOnly<u32>),
        (0x000C => _reserved0),
        (0x0040 => SETSPI_NSR: WriteOnly<u32>),
        (0x0044 => _reserved1),
        (0x0048 => CLRSPI_NSR: WriteOnly<u32>),
        (0x004C => _reserved2),
        (0x0050 => SETSPI_SR: WriteOnly<u32>),
        (0x0054 => _reserved3),
        (0x0058 => CLRSPI_SR: WriteOnly<u32>),
        (0x005C => _reserved4),
        (0x0080 => IGROUPR: [ReadWrite<u32>; 32]),
        (0x0100 => pub ISENABLER: [ReadWrite<u32>; 32]),
        (0x0180 => ICENABLER: [ReadWrite<u32>; 32]),
        (0x0200 => ISPENDR: [ReadWrite<u32>; 32]),
        (0x0280 => ICPENDR: [ReadWrite<u32>; 32]),
        (0x0300 => ISACTIVER: [ReadWrite<u32>; 32]),
        (0x0380 => ICACTIVER: [ReadWrite<u32>; 32]),
        (0x0400 => IPRIORITYR: [ReadWrite<u32>; 256]),
        (0x0800 => ITARGETSR: [ReadOnly<u32>; 256]),
        (0x0C00 => ICFGR: [ReadWrite<u32>; 64]),
        (0x0D00 => IGRPMODR: [ReadWrite<u32>; 64]),
        (0x0E00 => NSACR: [ReadWrite<u32>; 64]),
        (0x0F00 => SGIR: WriteOnly<u32>),
        (0x0F04 => _reserved5),
        (0x0F10 => CPENDSGIR: [ReadWrite<u32>; 4]),
        (0x0F20 => SPENDSGIR: [ReadWrite<u32>; 4]),
        (0x0F30 => _reserved6),
        (0x6100 => IROUTER: [ReadWrite<u64>; 960]),
        (0x7F00 => _reserved7),
        (0xC000 => ESTATUSR: ReadOnly<u32>),
        (0xC004 => ERRTESTR: WriteOnly<u32>),
        (0xC008 => _reserved8),
        (0xC084 => SPISR: [ReadOnly<u32>; 30]),
        (0xC0FC => _reserved9),
        (0xFFD0 => PIDRn: [ReadOnly<u32>; 12]),
        (0xFFFF => @END),
    }

}


register_structs! {
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub Redistributor_Register_Block {
        (0x0000 => CTLR: ReadWrite<u32>),
        (0x0004 => IIDR: ReadOnly<u32>),
        (0x0008 => TYPER: ReadOnly<u64, GICR_TYPER::Register>),
        (0x0010 => _reserved0),
        (0x0014 => WAKER: ReadWrite<u32, GICR_WAKER::Register>),
        (0x0018 => _reserved1),
        (0x0070 => PROPBASER: ReadWrite<u64>),
        (0x0078 => PENDBASER: ReadWrite<u64>),
        (0x0080 => _reserved2),
        (0xFFD0 => PIDRn: [ReadOnly<u32>; 8]),
        (0xFFF0 => CIDRn: [ReadOnly<u32>; 4]),
    // Redistributor_Register_For_SGIs_And_PPIs_Block
        (0x10000 => _reserved3),
        (0x10080 => IGROUPR0: ReadWrite<u32>),
        (0x10084 => _reserved4),
        (0x10100 => ISENABLER0: ReadWrite<u32>),
        (0x10104 => _reserved5),
        (0x10180 => ICENABLER0: ReadWrite<u32>),
        (0x10184 => _reserved6),
        (0x10200 => ISPENDR0: ReadWrite<u32>),
        (0x10204 => _reserved7),
        (0x10280 => ICPENDR0: ReadWrite<u32>),
        (0x10284 => _reserved8),
        (0x10300 => ISACTIVER0: ReadWrite<u32>),
        (0x10304 => _reserved9),
        (0x10380 => ICACTIVER0: ReadWrite<u32>),
        (0x10384 => _reserved10),
        (0x10400 => IPRIORITYR: [ReadWrite<u32>; 8]),
        (0x10420 => _reserved11),
        (0x10C00 => ICFGR_SGI: ReadWrite<u32>),
        (0x10C04 => ICFGR_PPI: ReadWrite<u32>),
        (0x10C08 => _reserved12),
        (0x10D00 => IGRPMODR0: ReadWrite<u32>),
        (0x10D04 => _reserved13),
        (0x10E00 => NSACR: ReadWrite<u32>),
        (0x10E04 => _reserved14),
        (0x1C000 => MISCSTATUSR: ReadWrite<u32>),
        (0x1C004 => _reserved15),
        (0x1C080 => PPISR: ReadWrite<u32>),
        (0x1C084 => _reserved16),
        (0x1FFFF => @END),
    }
}


register_structs! {
    #[allow(non_snake_case)]
    ITSControlRegisterBlock {
        (0x0000 => CTLR: ReadWrite<u32>),
        (0x0004 => IIDR: ReadOnly<u32>),
        (0x0008 => TYPER: ReadOnly<u64>),
        (0x0010 => _reserved0),
        (0x0080 => CBASER: ReadWrite<u64>),
        (0x0088 => CWRITER: ReadWrite<u64>),
        (0x0090 => CREADR: ReadOnly<u64>),
        (0x0098 => _reserved1),
        (0x0100 => BASER0: ReadWrite<u64>),
        (0x0108 => _reserved2),
        (0xC000 => TRKCTLR: WriteOnly<u32>),
        (0xC004 => TRKR: ReadOnly<u32>),
        (0xC008 => TRKDIDR: ReadOnly<u32>),
        (0xC00C => TRKPIDR: ReadOnly<u32>),
        (0xC010 => TRKVIDR: ReadOnly<u32>),
        (0xC014 => TRKTGTF: ReadWrite<u32>),
        (0xC018 => TRKICR: ReadWrite<u32>),
        (0xC01C => TRKLCR: ReadWrite<u32>),
        (0xC020 => _reserved3),
        (0xFFD0 => PIDR4: ReadOnly<u32>),
        (0xFFD4 => PIDR5: ReadOnly<u32>),
        (0xFFD8 => PIDR6: ReadOnly<u32>),
        (0xFFDC => PIDR7: ReadOnly<u32>),
        (0xFFE0 => PIDR0: ReadOnly<u32>),
        (0xFFE4 => PIDR1: ReadOnly<u32>),
        (0xFFE8 => PIDR2: ReadOnly<u32>),
        (0xFFEC => PIDR3: ReadOnly<u32>),
        (0xFFF0 => CIDR0: ReadOnly<u32>),
        (0xFFE4 => CIDR1: ReadOnly<u32>),
        (0xFFE8 => CIDR2: ReadOnly<u32>),
        (0xFFEC => CIDR3: ReadOnly<u32>),
        (0xFFFF => @END),
    }

}

type DistributorRegister = MMIODerefWrapper<Distributor_Register_Block>;
type RedistributorRegister = MMIODerefWrapper<Redistributor_Register_Block>;

pub struct GICInner {
    pub gicd: DistributorRegister,
    pub gicr: [RedistributorRegister; 6],
}

#[allow(dead_code)]
impl GICInner {
    /// Create an instance
    ///
    /// # Safety
    /// 
    /// prvide start address
    pub const unsafe fn new(gicd_start_addr: usize, gicr_start_addr: usize) -> Self {
        
        Self {
            gicd: DistributorRegister::new(gicd_start_addr),
            gicr: [RedistributorRegister::new(gicr_start_addr + 0x0_000),
                    RedistributorRegister::new(gicr_start_addr + 0x2_000),
                    RedistributorRegister::new(gicr_start_addr + 0x4_000),
                    RedistributorRegister::new(gicr_start_addr + 0x6_000),
                    RedistributorRegister::new(gicr_start_addr + 0x8_000),
                    RedistributorRegister::new(gicr_start_addr + 0xA_000),],
        }
    }

    pub unsafe fn init(&self) {

	// Global setting
        // Enable Affinity routing and all Groups
        self.gicd.CTLR.write(GICD_CTLR::ARE_NS::SET 
                            + GICD_CTLR::ARE_S::SET
                            + GICD_CTLR::EnableGrp1S::SET 
                            + GICD_CTLR::EnableGrp1NS::SET
                            + GICD_CTLR::EnableGrp0::SET
                            + GICD_CTLR::DS::CLEAR);

    // Redistributor configuration 
        self.gicr[0].WAKER.modify(GICR_WAKER::ProcessorSleep::CLEAR);
            
        // get current cpu
        let mut mpidr_el1: u64 = 0;
        asm!{
            "MRS {0}, MPIDR_EL1",
            out(reg) mpidr_el1,
        }
        
        // aff3:aff2:aff1:aff0
        let a = (((mpidr_el1 >> 32) & 0xF) << 24) | (mpidr_el1 & 0x0FFF);
        let mut cpu_num: usize = 0;
        loop {
            if a == self.gicr[cpu_num].TYPER.read(GICR_TYPER::Affinity_Value) >> 32 {
                break;
            }
            cpu_num += 1;
        }

        println!("cpu_num {}", cpu_num);

        while self.gicr[cpu_num].WAKER.matches_all(GICR_WAKER::ChildrenAsleep::SET) {
            self.gicr[cpu_num].WAKER.modify(GICR_WAKER::ChildrenAsleep::CLEAR);
            barrier::dsb(barrier::SY);
        }

    // CPU interface configuration
        //crate::arch::boot::cpu_interface_init();

    // SPI, PPI and SGI configuration
        // Priority
        // self.gicd.IPRIORITYR
        // self.sgi_ppi.IPRIORITYR

        // Group, Config SPIs as Group1
        let nums = self.gicd.TYPER.read(GICD_TYPER::ITLinesNumber) as usize;
        println!("nums {}", nums);
        for i in 0..nums {
            self.gicd.IGROUPR[i].set(!0x0);
            self.gicd.IGRPMODR[i].set(!0x0);
        }

        // Edge-triggered/level-sensitive 
        // for physical signal

        // Enable
        //self.gicr[cpu_num].IGROUPR0.set(0x0);
        //self.gicr[cpu_num].IGRPMODR0.set(0x0);    //SGIs|PPIs Group1NS 
        self.gicr[cpu_num].ISENABLER0.set(0x01);  //Enable SGI 0 
        //self.gicr[cpu_num].ICENABLER0.set(0x0);

    }

}
