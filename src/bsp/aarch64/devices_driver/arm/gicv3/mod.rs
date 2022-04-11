
mod regs;

use cortex_a::registers::*;
use tock_registers::interfaces::{ReadWriteable, Writeable};
use crate::println;
use crate::bsp::board::map::{GICD_BASE, GICR_BASE};

type InterruptHandler = fn();

const NR_GIC_IRQS: usize = 6 * 32;
const NR_GPIO_IRQS: usize = 5 * 32;
const NR_IRQS: usize = NR_GIC_IRQS + NR_GPIO_IRQS;

// irq handler array
static mut IRQ_HANDLER:[Option<InterruptHandler>; NR_IRQS] = [None; NR_IRQS];
// gicv3
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

#[inline(always)]
pub fn irq_handler_enable(irq: usize) {

    let m = irq / 32;
    let n = irq % 32;

    GIC.gicd.ISENABLER[m].set(0x1 << n);
    
}

#[no_mangle]
pub unsafe fn irq_handler() {
    
    let mut intid = 0;
    asm!(
	    "MRS {0}, ICC_IAR1_EL1",
        out(reg) intid,
    );
    intid %= 0x3FF; 

    if intid < NR_GIC_IRQS {
        if let Some(handler) = IRQ_HANDLER[intid] {
            handler();
        } else {
            println!("no handler id:{}", intid);
        }
    } else {
        println!("error nintid {}", intid);
    }

    asm!(
	    "MSR ICC_EOIR1_EL1, {0}",
	    "MSR ICC_DIR_EL1, {0}",
        "ISB SY",
        in(reg) intid as u64,
    );
}

