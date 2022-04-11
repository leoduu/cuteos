
pub mod gic;

// Assembly counterpart to this file.
global_asm!(include_str!("interrupt.S"));

extern "C" {
    pub fn read_icc_iar1_el1() -> usize;
    pub fn write_icc_iar1_el1(x0: u64);
    pub fn read_spsr_el1() -> usize;
}

use cortex_a::{asm::barrier, registers::*};
use tock_registers::interfaces::{ReadWriteable, Writeable};
use crate::drivers::Driver;
use crate::{println};
use crate::bsp::map::physical::{GICD_BASE, GICR_BASE};


type InterruptHandler = fn();

const NR_GIC_IRQS: usize = 6 * 32;
const NR_GPIO_IRQS: usize = 5 * 32;
const NR_IRQS: usize = NR_GIC_IRQS + NR_GPIO_IRQS;
const TIMER3_INTR: usize = 116;

static mut IRQ_HANDLER:[Option<InterruptHandler>; NR_IRQS] = [None; NR_IRQS];

static GIC: gic::GICInner = unsafe{ gic::GICInner::new(GICD_BASE, GICR_BASE) };

pub fn gic() -> &'static gic::GICInner {
    &GIC
}

#[inline(always)]
pub unsafe fn enable_interrupt() {
    DAIF.modify(DAIF::I::Unmasked);
}

#[inline(always)]
pub unsafe fn disable_interrupt() {
    DAIF.modify(DAIF::I::Masked);
}

#[inline(always)]
pub unsafe fn irq_install_handler(irq: usize, handler: InterruptHandler)
{
    IRQ_HANDLER[irq] = Some(handler);
}

pub fn irq_handler_enable(irq: usize) -> Result<(), &'static str> {
    if irq >= NR_GIC_IRQS {
        return Err("irq bigger than NR_GIC_IRQS");
    } 

    let m = irq / 32;
    let n = irq % 32;

    GIC.gicd.ISENABLER[m].set(0x1 << n);
    Ok(())
}

#[no_mangle]
pub unsafe fn do_irq() {

    println!("do_irq");
    println!("{}", crate::arch::boot::current_privilege_level_str());
    
    let mut irqstat: usize = TIMER3_INTR;// = raw_read_icc_iar1_el1();
    asm!(
	    "MRS {0}, ICC_IAR1_EL1",
        "dsb sy",
        out(reg) irqstat,
    );

    let nintid = irqstat;// % 0x03FF;

    if nintid < NR_GIC_IRQS {
        if let Some(handler) = IRQ_HANDLER[nintid] {
            println!("handler id:{} addr:{:?}", nintid, handler);
            handler();
        } else {
            println!("no handler id:{}", nintid);
        }
    } else {
        println!("error nintid {}", nintid);
    }

    asm!(
	    "MSR ICC_EOIR1_EL1, {0}",
	    "MSR ICC_DIR_EL1, {0}",
        "isb sy",
        in(reg) nintid as u64,
    );
    let timer3 = crate::drivers::timer::timer(3);
    timer3.clear_irq();
    //write_icc_iar1_el1(nintid as u64);
}


#[no_mangle]
pub fn timer_handler() {

    let timer3 = crate::drivers::timer::timer(3);
    timer3.clear_irq();

    println!("timer_handler");
}

pub fn timer3_interrupt_test() {

    let timer3 = crate::drivers::timer::timer(3);
    unsafe{
        timer3.init().unwrap();
        irq_install_handler(TIMER3_INTR, timer_handler);
        irq_handler_enable(TIMER3_INTR).unwrap();
    }

    println!("wait for timer3 irq");
    timer3.irq_delay_us(1000_000);
}
