// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2021 Andre Richter <andre.o.richter@gmail.com>

//--------------------------------------------------------------------------------------------------
// Definitions
//--------------------------------------------------------------------------------------------------

// Load the address of a symbol into a register, PC-relative.
//
// The symbol must lie within +/- 4 GiB of the Program Counter.
//
// # Resources
//
// - https://sourceware.org/binutils/docs-2.36/as/AArch64_002dRelocations.html
.macro ADR_REL register, symbol
	adrp	\register, \symbol
	add	\register, \register, #:lo12:\symbol
.endm

// Load the address of a symbol into a register, absolute.
//
// # Resources
//
// - https://sourceware.org/binutils/docs-2.36/as/AArch64_002dRelocations.html
.macro ADR_ABS register, symbol
	movz	\register, #:abs_g2:\symbol
	movk	\register, #:abs_g1_nc:\symbol
	movk	\register, #:abs_g0_nc:\symbol
.endm

/*
 * Branch according to exception level
 */
.macro	switch_el, xreg, el3_label, el2_label, el1_label
	mrs	\xreg, CurrentEL
	cmp	\xreg, 0xc
	b.eq	\el3_label
	cmp	\xreg, 0x8
	b.eq	\el2_label
	cmp	\xreg, 0x4
	b.eq	\el1_label
.endm


.equ _core_id_mask, 0b11
.equ _core_frequence, 24000000

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------
.section .text._start

//------------------------------------------------------------------------------
// fn _start()
//------------------------------------------------------------------------------
_start:
	// Only proceed on the boot core. Park it otherwise.
	mrs	x1, MPIDR_EL1
	and	x1, x1, _core_id_mask
	ldr	x2, BOOT_CORE_ID      // provided by bsp/__board_name__/cpu.rs
	cmp	x1, x2
	b.ne	.L_parking_loop

	// If execution reaches here, it is the boot core.

	// set core_frequence for rk3399
	switch_el x0, 3f, 2f, 1f
3:
	ldr	x0, =_core_frequence           //24MHz
	msr	cntfrq_el0, x0			//Initialize CNTFRQ 
2:
1:

	// Initialize DRAM.
	ADR_ABS	x0, __bss_start
	ADR_ABS x1, __bss_end_exclusive

.L_bss_init_loop:
	cmp	x0, x1
	b.eq	.L_relocate_binary
	stp	xzr, xzr, [x0], #16
	b	.L_bss_init_loop

 
	// Next, relocate the binary.
.L_relocate_binary:
	ADR_REL	x0, __binary_nonzero_start         // The address the binary got loaded to.
	ADR_ABS	x1, __binary_nonzero_start         // The address the binary was linked to.
	ADR_ABS	x2, __binary_nonzero_end_exclusive

.L_copy_loop:
	ldr	x3, [x0], #8
	str	x3, [x1], #8
	cmp	x1, x2
	b.lo	.L_copy_loop

	// Prepare the jump to Rust code.
	// Set the stack pointer.
	ADR_ABS	x0, __boot_core_stack_end_exclusive
	mov	sp, x0

	// Jump to the relocated Rust code.
	b 	_start_rust

	// Infinitely wait for events (aka "park the core").
.L_parking_loop:
	wfe
	b	.L_parking_loop

.size	_start, . - _start
.type	_start, function
.global	_start



/*******************************************************/	
.global lowlevel_init
lowlevel_init:
	mov	x29, lr		             //Save LR 

	ldr	x0, =0xFEE00000          //RKIO_GICD_PHYS   GIC DIST 
	bl	gic_init_secure

	ldr	x0, =0xFEF00000          //RKIO_GICR_PHYS
	bl	gic_init_secure_percpu

	mov	lr, x29			         //Restore LR
	ret	
	
/*******************************************************/	
//ref: u-boot/arch/arm/lib/gic_64.S

	/*Initialize Distributor  x0: Distributor Base*/
gic_init_secure:
	mov	w9, #0x37		         //EnableGrp0 | EnableGrp1NS  
				            	 //EnableGrp1S | ARE_S | ARE_NS  
	str	w9, [x0, 0x0000]	     //Secure GICD_CTLR 
	ldr	w9, [x0, 0x0004]
	and	w10, w9, #0x1f		     //ITLinesNumber 
	cbz	w10, 1f			         //No SPIs 
	add	x11, x0, (0x0080 + 4)
	add	x12, x0, (0x0d00 + 4)
	mov	w9, #~0
0:	str	w9, [x11], #0x4
	str	wzr, [x12], #0x4	     //Config SPIs as Group1NS 
	sub	w10, w10, #0x1
	cbnz	w10, 0b
1:
	ret
	
	
	/*Initialize ReDistributor  x0: ReDistributor Base*/
gic_init_secure_percpu:
	mrs	x10, mpidr_el1
	lsr	x9, x10, #32
	bfi	x10, x9, #24, #8	    //w10 is aff3:aff2:aff1:aff0 
	mov	x9, x0
1:	ldr	x11, [x9, 0x0008]
	lsr	x11, x11, #32		    //w11 is aff3:aff2:aff1:aff0 
	cmp	w10, w11
	b.eq	2f
	add	x9, x9, #(2 << 16)
	b	1b

	/* x9: ReDistributor Base Address of Current CPU */
2:	mov	w10, #~0x2
	ldr	w11, [x9, 0x0014]
	and	w11, w11, w10		    //Clear ProcessorSleep 
	str	w11, [x9, 0x0014]
	dsb	st
	isb
3:	ldr	w10, [x9, 0x0014]
	tbnz	w10, #2, 3b		    //Wait Children be Alive 

	add	x10, x9, #(1 << 16)   	//SGI_Base 
	mov	w11, #~0
	str	w11, [x10, 0x0080]
	str	wzr, [x10, 0x0d00]	    //SGIs|PPIs Group1NS 
	mov	w11, #0x1		        //Enable SGI 0 
	str	w11, [x10, 0x0100]

	/* Initialize Cpu Interface */
	/* rockchip: first check elx for running on different el */
	switch_el x0, el3_sre, el2_sre, el1_sre

el3_sre:
	mrs	x10, S3_6_C12_C12_5
	orr	x10, x10, #0xf		     //SRE & Disable IRQ/FIQ Bypass & 
					             //Allow EL2 access to ICC_SRE_EL2 
	msr	S3_6_C12_C12_5, x10
	isb

el2_sre:
	mrs	x10, S3_4_C12_C9_5
	orr	x10, x10, #0xf		     //SRE & Disable IRQ/FIQ Bypass & 
				             	 //Allow EL1 access to ICC_SRE_EL1 
	msr	S3_4_C12_C9_5, x10
	isb

el1_sre:
	mrs	x0, CurrentEL	         //check currentEL 
	cmp	x0, 0xC
	b.ne	el1_ctlr	         //currentEL != EL3 

el3_ctlr:
	mov	x10, #0x3		         //EnableGrp1NS | EnableGrp1S 
	msr	S3_6_C12_C12_7, x10
	isb

	msr	S3_6_C12_C12_4, xzr
	isb

el1_ctlr:
	mov	x10, #0x3		         //EnableGrp1NS | EnableGrp1S 
	msr	S3_0_C12_C12_7, x10
	isb

	msr	S3_0_C12_C12_4, xzr    	 //NonSecure ICC_CTLR_EL1 
	isb

	mov	x10, #0xf0		         //Non-Secure access to ICC_PMR_EL1 
	msr	S3_0_C4_C6_0, x10
	isb	

	ret

