// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2021 Andre Richter <andre.o.richter@gmail.com>

//--------------------------------------------------------------------------------------------------
// Definitions
//--------------------------------------------------------------------------------------------------

.global
.macro STORE_CONTEXT 
	// Make room on the stack for the exception conteXt.
	SUB	SP,  SP,  #0x10 * 17

	// Store all general purpose registers on the stack.
	STP	X0,  X1,  [SP, #0x10 * 0]
	STP	X2,  X3,  [SP, #0x10 * 1]
	STP	X4,  X5,  [SP, #0x10 * 2]
	STP	X6,  X7,  [SP, #0x10 * 3]
	STP	X8,  X9,  [SP, #0x10 * 4]
	STP	X10, X11, [SP, #0x10 * 5]
	STP	X12, X13, [SP, #0x10 * 6]
	STP	X14, X15, [SP, #0x10 * 7]
	STP	X16, X17, [SP, #0x10 * 8]
	STP	X18, X19, [SP, #0x10 * 9]
	STP	X20, X21, [SP, #0x10 * 10]
	STP	X22, X23, [SP, #0x10 * 11]
	STP	X24, X25, [SP, #0x10 * 12]
	STP	X26, X27, [SP, #0x10 * 13]
	STP	X28, X29, [SP, #0x10 * 14]

	// Add the eXception link register (ELR_EL1), saved program status (SPSR_EL1) and eXception
	// syndrome register (ESR_EL1).
	MRS	X1,  ELR_EL1
	MRS	X2,  SPSR_EL1
	MRS	X3,  ESR_EL1

	STP	LR,  X1,  [SP, #0x10 * 15]
	STP	X2,  X3,  [SP, #0x10 * 16]

	// X0 is the first argument for the function called through `\handler`.
	MOV	X0,  SP
.endm

.macro RESTORE_CONTEXT

	LDR	X19,      [SP, #0x10 * 16]
	LDP	LR,  X20, [SP, #0x10 * 15]

	MSR	SPSR_EL1, X19
	MSR	ELR_EL1,  X20

	LDP	X0,  X1,  [SP, #0x10 * 0]
	LDP	X2,  X3,  [SP, #0x10 * 1]
	LDP	X4,  X5,  [SP, #0x10 * 2]
	LDP	X6,  X7,  [SP, #0x10 * 3]
	LDP	X8,  X9,  [SP, #0x10 * 4]
	LDP	X10, X11, [SP, #0x10 * 5]
	LDP	X12, X13, [SP, #0x10 * 6]
	LDP	X14, X15, [SP, #0x10 * 7]
	LDP	X16, X17, [SP, #0x10 * 8]
	LDP	X18, X19, [SP, #0x10 * 9]
	LDP	X20, X21, [SP, #0x10 * 10]
	LDP	X22, X23, [SP, #0x10 * 11]
	LDP	X24, X25, [SP, #0x10 * 12]
	LDP	X26, X27, [SP, #0x10 * 13]
	LDP	X28, X29, [SP, #0x10 * 14]

	ADD	SP,  SP,  #0x10 * 17

	ERET
.endm

.macro CALL_WITH handler

	STORE_CONTEXT
	
	MOV	X0,  SP		// param ESR_EL1
	BL \handler
	
	RESTORE_CONTEXT

.endm

.section .text.
/* Exception vectors.*/
	.align	11
__exception_vector:
.align	7
	b 	do_el0_sync	 		//Current EL0 Synchronous Thread                
.align	7              
	b	do_el0_irq				//Current EL0 IRQ Thread                        
.align	7               
	b	do_el0_fiq			//Current EL0 FIQ Thread 
.align	7               
	b	do_el0_error		//Current EL0 SError Thread 
                           
// Current exception level with SP_ELx, x > 0.
.align	7               
	b	do_elx_sync	   		//Current ELx Synchronous Handler                       
.align	7               
	b	do_elx_irq		  		//Current ELx IRQ Handler                        
.align	7               
	b	do_elx_fiq		   	//Current ELx FIQ Handler                  
.align	7              
	b	do_error	        //Current ELx Error Handler 

// Lower exception level, AArch64
.align	7               
	b	do_low64_sync	  	                    
.align	7               
	b	do_low64_irq		                    
.align	7               
	b	do_low64_fiq		                
.align	7              
	b	do_low64_error	  

// Lower exception level, AArch32
.align	7               
	b	do_low32_sync	  	                   
.align	7               
	b	do_low32_irq		                      
.align	7               
	b	do_low32_fiq		              
.align	7              
	b	do_low32_error	  	


do_el0_sync:	    
	CALL_WITH	default_sync
do_el0_irq:			
	CALL_WITH 	default_irq         
do_el0_fiq:			
	CALL_WITH	default_fiq
do_el0_error:		
	CALL_WITH	default_serror

// Current exception level with SP_ELx, x > 0.
do_elx_sync:	   	
	CALL_WITH	default_sync          
do_elx_irq:		  
	CALL_WITH 	default_irq                
do_elx_fiq:		   
	CALL_WITH	default_fiq          
do_error:	       
	CALL_WITH	default_serror     

// Lower exception level, AArch64
do_low64_sync:	  	                 
	CALL_WITH	default_sync
do_low64_irq:	
	CALL_WITH 	default_irq
	//b 	do_irq                  
do_low64_fiq:	
	CALL_WITH	default_fiq              
do_low64_error:	 
	CALL_WITH	default_serror               
 
// Lower exception level, AArch32
do_low32_sync:	       
	CALL_WITH	default_sync            
do_low32_irq:	
	CALL_WITH 	default_irq                
do_low32_fiq:	
	CALL_WITH	default_fiq              
do_low32_error:	 
	CALL_WITH	default_serror


halt:
	B  halt 

