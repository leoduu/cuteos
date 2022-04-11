
.global do_irq 
do_irq:

	// Store all general purpose registers on the stack.
	sub		sp,  sp,  #0x10 * 17
	stp		x0,  x1,  [sp, #0x10 * 0]
	stp		x2,  x3,  [sp, #0x10 * 1]
	stp		x4,  x5,  [sp, #0x10 * 2]
	stp		x6,  x7,  [sp, #0x10 * 3]
	stp		x8,  x9,  [sp, #0x10 * 4]
	stp		x10, x11, [sp, #0x10 * 5]
	stp		x12, x13, [sp, #0x10 * 6]
	stp		x14, x15, [sp, #0x10 * 7]
	stp		x16, x17, [sp, #0x10 * 8]
	stp		x18, x19, [sp, #0x10 * 9]
	stp		x20, x21, [sp, #0x10 * 10]
	stp		x22, x23, [sp, #0x10 * 11]
	stp		x24, x25, [sp, #0x10 * 12]
	stp		x26, x27, [sp, #0x10 * 13]
	stp		x28, x29, [sp, #0x10 * 14]

	mrs 	x1, sp_el0
	stp		lr,  x1,  [sp, #0x10 * 15]

	mrs		x1,	elr_el1
	mrs		x2, spsr_el1
	stp		x1,  x2,  [sp, #0x10 * 16]

	// restore kernel sp
	ldr 	x1,		  [sp, #0x10 * 17]
	mov 	sp, x1

	ldp     x19, x20, [sp, #0x10 * 0]
	ldp	    x21, x22, [sp, #0x10 * 1]
	ldp	    x23, x24, [sp, #0x10 * 2]
	ldp	    x24, x26, [sp, #0x10 * 3]
	ldp	    x27, x28, [sp, #0x10 * 4]
	ldp	    x29, x30, [sp, #0x10 * 5]
    add     sp,  sp,  #0x10 * 6

    bl irq_handler 

    sub     sp,  sp,  #0x10 * 6
	stp     x19, x20, [sp, #0x10 * 0]
	stp	    x21, x22, [sp, #0x10 * 1]
	stp	    x23, x24, [sp, #0x10 * 2]
	stp	    x24, x26, [sp, #0x10 * 3]
	stp	    x27, x28, [sp, #0x10 * 4]
	stp	    x29, x30, [sp, #0x10 * 5]

	// save kernel sp
	mov		x1, sp
	// change to process StackFrame
	mov		sp, x0
	str 	x1, 	  [sp, #0x10 * 17]

	// load sp
	ldp		x1,  x2,  [sp, #0x10 * 16]
	msr		elr_el1,  x1
	msr		spsr_el1, x2

	// load elr and spsr
	ldp		lr,  x1,  [sp, #0x10 * 15]
	msr		sp_el0,   x1

	ldp		x0,  x1,  [sp, #0x10 * 0]
	ldp		x2,  x3,  [sp, #0x10 * 1]
	ldp		x4,  x5,  [sp, #0x10 * 2]
	ldp		x6,  x7,  [sp, #0x10 * 3]
	ldp		x8,  x9,  [sp, #0x10 * 4]
	ldp		x10, x11, [sp, #0x10 * 5]
	ldp		x12, x13, [sp, #0x10 * 6]
	ldp		x14, x15, [sp, #0x10 * 7]
	ldp		x16, x17, [sp, #0x10 * 8]
	ldp		x18, x19, [sp, #0x10 * 9]
	ldp		x20, x21, [sp, #0x10 * 10]
	ldp		x22, x23, [sp, #0x10 * 11]
	ldp		x24, x25, [sp, #0x10 * 12]
	ldp		x26, x27, [sp, #0x10 * 13]
	ldp		x28, x29, [sp, #0x10 * 14]

	add		sp,  sp,  #0x10 * 17
	eret


// x0 &mut sp (to)
.global os_cpu_switch
os_cpu_switch: 

	ret


// x0 &mut sp (to)
.global os_cpu_switch_to
os_cpu_switch_to:

	// save callee-saved registers x19-x29
    sub     sp,  sp,  #0x10 * 6
	stp     x19, x20, [sp, #0x10 * 0]
	stp	    x21, x22, [sp, #0x10 * 1]
	stp	    x23, x24, [sp, #0x10 * 2]
	stp	    x24, x26, [sp, #0x10 * 3]
	stp	    x27, x28, [sp, #0x10 * 4]
	stp	    x29, x30, [sp, #0x10 * 5]

	// save kernel sp
	mov		x1, sp
	// change to process StackFrame
	mov		sp, x0
	str 	x1, 	  [sp, #0x10 * 17]

	// load sp
	ldp		x1,  x2,  [sp, #0x10 * 16]
	msr		elr_el1,  x1
	msr		spsr_el1, x2

	// load elr and spsr
	ldp		lr,  x1,  [sp, #0x10 * 15]
	msr		sp_el0,   x1

	// load general purpose registers
	ldp		x0,  x1,  [sp, #0x10 * 0]
	ldp		x2,  x3,  [sp, #0x10 * 1]
	ldp		x4,  x5,  [sp, #0x10 * 2]
	ldp		x6,  x7,  [sp, #0x10 * 3]
	ldp		x8,  x9,  [sp, #0x10 * 4]
	ldp		x10, x11, [sp, #0x10 * 5]
	ldp		x12, x13, [sp, #0x10 * 6]
	ldp		x14, x15, [sp, #0x10 * 7]
	ldp		x16, x17, [sp, #0x10 * 8]
	ldp		x18, x19, [sp, #0x10 * 9]
	ldp		x20, x21, [sp, #0x10 * 10]
	ldp		x22, x23, [sp, #0x10 * 11]
	ldp		x24, x25, [sp, #0x10 * 12]
	ldp		x26, x27, [sp, #0x10 * 13]
	ldp		x28, x29, [sp, #0x10 * 14]

	add		sp,  sp,  #0x10 * 17

	// return
	eret
