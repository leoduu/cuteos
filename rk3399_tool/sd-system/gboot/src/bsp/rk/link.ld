
/* The address at which the kernel binary will be loaded */
__rk3399_load_addr = 0x80000;

OUTPUT_ARCH(aarch64)
ENTRY(__rk3399_load_addr)

PHDRS
{
    segment_rx PT_LOAD FLAGS(5); /* 5 == RX */
    segment_rw PT_LOAD FLAGS(6); /* 6 == RW */
}

SECTIONS 
{
    /* Set the link address to 32 MiB */
	. = 0x2000000;

	__boot_core_stack_end_exclusive = .;

    /* Code + RO Data */    
    __binary_nonzero_start = .;
    .text :
    {
        KEEP(*(.text._start))
        *(.text._start_arguments) /* Constants (or statics in Rust speak) read by _start(). */
        *(.text._start_rust)      /* The Rust entry point */
        *(.text*)                 /* Everything else */
    } :segment_rx

    .rodata : ALIGN(8) { *(.rodata*)} :segment_rx
    .got    : ALIGN(8) { *(.got)    } :segment_rx


    /* Data + BSS */
    .data   : ALIGN(8) { *(.data*)   } :segment_rw
    
    . = ALIGN(8);
    __binary_nonzero_end_exclusive = .;

    .bss    : ALIGN(16)  
    {  
        __bss_start = .;
        *(.bss*);
        . = ALIGN(16); 
        __bss_end_exclusive = .;
    } :NONE
}
