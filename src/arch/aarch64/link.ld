/* SPDX-License-Identifier: MIT OR Apache-2.0
 *
 * Copyright (c) 2018-2021 Andre Richter <andre.o.richter@gmail.com>
 */

/* The address at which the the kernel binary will be loaded by the Raspberry's firmware */
__kernel_load_addr = 0x80000;

ENTRY(__kernel_load_addr)

SECTIONS
{
    . =  __kernel_load_addr;
                                        /*   ^             */
                                        /*   | stack       */
                                        /*   | growth      */
                                        /*   | direction   */
    __stack_end_exclusive = .; /*   |             */

    /***********************************************************************************************
    * Heap
    ***********************************************************************************************/
    __heap_start = .;
    .heap   : ALIGN(16) { *(.heap.*) }
    __heap_end = .;

    /***********************************************************************************************
    * Code + RO Data + Global Offset Table
    ***********************************************************************************************/
    __rx_start = .;
    .text :
    {
        KEEP(*(.text._start))
        *(.text._start_arguments) /* Constants (or statics in Rust speak) read by _start(). */
        *(.text._start_rust)      /* The Rust entry point */
        *(.text*)                 /* Everything else */
    } 

    .rodata : ALIGN(8) { *(.rodata*) } 
    .got    : ALIGN(8) { *(.got)     } 

    . = ALIGN(64K); /* Align to page boundary */
    __rx_end_exclusive = .;

    /***********************************************************************************************
    * Data + BSS
    ***********************************************************************************************/
    .data : { *(.data*) } 

    /* Section is zeroed in pairs of u64. Align start and end to 16 bytes */
    .bss : ALIGN(16)
    {
        __bss_start = .;
        *(.bss*);
        . = ALIGN(16);
        __bss_end_exclusive = .;
    } 
}
