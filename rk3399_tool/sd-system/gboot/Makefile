
BSP ?= firefly-rk3399

# Default to a serial device name that is common in Linux
DEV_SERIAL ?= /dev/ttyUSB0

#---------------------------------------------------------------------
# Hardcoded configuation values
#---------------------------------------------------------------------
# common parameter
	TARGET 			= aarch64-unknown-none-softfloat
	KERNEL_BIN  	= ./gboot.bin
	KERNEL_DIS		= gboot.dis
	OBJDUMP			= aarch64-none-elf-objdump
	NM		      	= aarch64-none-elf-nm
	READELF		 	= aarch64-none-elf-readelf
	RUSTC_MISC_ARGS	= -C target-cpu=cortex-a72

ifeq ($(BSP), firefly-rk3399)
	LINKER_FILE    	= src/bsp/rk/link.ld
	BOOT_MERGER		= ../tools/loaderimage
	BOOT_IMG		= ../gboot.img
	BOOT_FLAGS  	= --pack --uboot
else ifeq ($(BSP),rpi4)
	LINKER_FILE    	= src/bsp/raspberrypi/link.ld
endif


export LINKER_FILE

KERNEL_ELF = target/$(TARGET)/debug/kernel

#---------------------------------------------------------------------
# Command building blocks
#---------------------------------------------------------------------
RUSTFLAGS			= -C link-arg=-T$(LINKER_FILE) $(RUSTC_MISC_ARGS)
RUSTFLAGS_PEDANTIC	= $(RUSTFLAGS) #-D warnings -D missing_docs

FEATURES		= --features bsp_$(BSP)
COMPILER_ARGS 	= --target=$(TARGET) \
    $(FEATURES)                    	 \
#    --release

RUSTC_CMD   = cargo rustc 	$(COMPILER_ARGS)
DOC_CMD     = cargo doc 	$(COMPILER_ARGS)
CLIPPY_CMD  = cargo clippy 	$(COMPILER_ARGS)
CHECK_CMD   = cargo check 	$(COMPILER_ARGS)
OBJCOPY_CMD = rust-objcopy --strip-all -O binary
OBJDUMP_CMD = rust-objdump -D 

##--------------------------------------------------------------------------------------------------
## Targets
##--------------------------------------------------------------------------------------------------
.PHONY: all $(KERNEL_ELF) $(KERNEL_BIN) clippy clean readelf objdump check

ifeq ($(BSP),firefly-rk3399)
all: $(BOOT_IMG)
else ifeq ($(BSP),rpi4)
all: $(KERNEL_BIN)
endif

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

$(BOOT_IMG): $(KERNEL_BIN)
	@echo "\n======== qboot merger ========"
	@$(BOOT_MERGER) $(BOOT_FLAGS) $(KERNEL_BIN) $(BOOT_IMG)

##------------------------------------------------------------------------------
## Build the documentation
##------------------------------------------------------------------------------
doc:
	$(call colorecho, "\nGenerating docs")
	@$(DOC_CMD) --document-private-items --open

##------------------------------------------------------------------------------
## Run clippy
##------------------------------------------------------------------------------
clippy:
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(CLIPPY_CMD)

##------------------------------------------------------------------------------
## Clean
##------------------------------------------------------------------------------
clean:
	rm -rf target $(BOOT_TEMP) $(BOOT_SD_BIN) $(KERNEL_BIN) $(KERNEL_DIS)

##------------------------------------------------------------------------------
## Helper target for rust-analyzer
##------------------------------------------------------------------------------
check:
	RUSTFLAGS="$(RUSTFLAGS)" $(CHECK_CMD) --message-format=json


