
BOARD ?= raspi4

# Default to a serial device name that is common in Linux
DEV_SERIAL ?= /dev/ttyUSB0

#---------------------------------------------------------------------
# Hardcoded configuation values
#---------------------------------------------------------------------
# common parameter

# global
	KERNEL_BIN  		= kernel.bin
	KERNEL_DIS			= kernel.dis
ifeq ($(BOARD), firefly-rk3399)
	TARGET 				= aarch64-unknown-none-softfloat
	OBJDUMP				= aarch64-none-elf-objdump
	NM		      		= aarch64-none-elf-nm
	READELF		 		= aarch64-none-elf-readelf
	LINKER_FILE    		= src/arch/aarch64/link.ld
	QEMU_BINARY       	= qemu-system-aarch64
    QEMU_MACHINE_TYPE 	= 
    QEMU_RELEASE_ARGS 	= -serial stdio -display none
	RUSTC_MISC_ARGS 	= -C target-cpu=cortex-a72
else ifeq ($(BOARD), raspi3)
	TARGET 				= aarch64-unknown-none-softfloat
	OBJDUMP				= aarch64-none-elf-objdump
	NM		      		= aarch64-none-elf-nm
	READELF		 		= aarch64-none-elf-readelf
	LINKER_FILE    		= src/arch/aarch64/link.ld
	QEMU_BINARY       	= qemu-system-aarch64
    QEMU_MACHINE_TYPE 	= raspi3b
    QEMU_ARGS 			= -serial stdio -display none
	RUSTC_MISC_ARGS 	= -C target-cpu=cortex-a53
else ifeq ($(BOARD), raspi4)
	TARGET 				= aarch64-unknown-none-softfloat
	OBJDUMP				= aarch64-none-elf-objdump
	NM		      		= aarch64-none-elf-nm
	READELF		 		= aarch64-none-elf-readelf
	LINKER_FILE    		= src/arch/aarch64/link.ld
	QEMU_BINARY       	= qemu-system-aarch64
    QEMU_MACHINE_TYPE 	= 
    QEMU_ARGS 			= -serial stdio -display none
	RUSTC_MISC_ARGS 	= -C target-cpu=cortex-a72
endif


export LINKER_FILE

#KERNEL_ELF = target/$(TARGET)/release/kernel
KERNEL_ELF = target/$(TARGET)/debug/kernel

#---------------------------------------------------------------------
# Command building blocks
#---------------------------------------------------------------------
RUSTFLAGS			= -C link-arg=-T$(LINKER_FILE) $(RUSTC_MISC_ARGS)
RUSTFLAGS_PEDANTIC	= $(RUSTFLAGS) #-D warnings -D missing_docs

FEATURES		= --features board_$(BOARD) 
COMPILER_ARGS 	= --target=$(TARGET) \
    $(FEATURES)                    	 \
    #--release

RUSTC_CMD   = cargo rustc $(COMPILER_ARGS)
DOC_CMD     = cargo doc $(COMPILER_ARGS)
TEST_CMD  	= cargo test $(COMPILER_ARGS)
CHECK_CMD   = cargo check $(COMPILER_ARGS)
CLIPPY_CMD  = cargo clippy $(COMPILER_ARGS)
OBJCOPY_CMD = rust-objcopy --strip-all -O binary
OBJDUMP_CMD = rust-objdump -D 
READELF_CMD = aarch64-linux-gnu-readelf -a
READELF_OUTPUT = ./kernel.elf

##--------------------------------------------------------------------------------------------------
## Targets
##--------------------------------------------------------------------------------------------------
.PHONY: all $(KERNEL_ELF) $(KERNEL_BIN) test clean readelf objdump check clippy

all: $(KERNEL_BIN)

##------------------------------------------------------------------------------
## Build the kernel ELF
##------------------------------------------------------------------------------
$(KERNEL_ELF):
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(RUSTC_CMD)

##------------------------------------------------------------------------------
## Build the stripped kernel binary
##------------------------------------------------------------------------------
$(KERNEL_BIN): $(KERNEL_ELF)
	@$(OBJCOPY_CMD) $(KERNEL_ELF) $(KERNEL_BIN)
	@$(OBJDUMP_CMD) $(KERNEL_ELF) > $(KERNEL_DIS)
	@readelf -a $(KERNEL_ELF) > kernel.elf
	@echo
	@ls -l  $(KERNEL_BIN)
	@ls -lh $(KERNEL_BIN)
	@echo

##------------------------------------------------------------------------------
## Run the kernel in QEMU
##------------------------------------------------------------------------------
ifeq ($(QEMU_MACHINE_TYPE),) # QEMU is not supported for the board.

qemu:
	@echo $(QEMU_MISSING_STRING)

else # QEMU is supported.

qemu: $(KERNEL_BIN)
	@echo "Launching QEMU"
	@$(QEMU_BINARY) -M $(QEMU_MACHINE_TYPE) $(QEMU_ARGS) -kernel $(KERNEL_BIN)

endif

##------------------------------------------------------------------------------
## Build the documentation
##------------------------------------------------------------------------------
doc:
	$(call colorecho, "\nGenerating docs")
	@$(DOC_CMD) --document-private-items --open

##------------------------------------------------------------------------------
## Run test
##------------------------------------------------------------------------------
test:
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(TEST_CMD)

##------------------------------------------------------------------------------
## Clean
##------------------------------------------------------------------------------
clean:
	rm -rf target $(KERNEL_BIN) $(KERNEL_DIS) kernel.elf

readelf:
	$(READELF_CMD) $(KERNEL_ELF) > $(READELF_OUTPUT)

##------------------------------------------------------------------------------
## Run clippy
##------------------------------------------------------------------------------
clippy:
	@RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(CLIPPY_CMD)

##------------------------------------------------------------------------------
## Helper target for rust-analyzer
##------------------------------------------------------------------------------
check:
	RUSTFLAGS="$(RUSTFLAGS)" $(CHECK_CMD) --message-format=json


