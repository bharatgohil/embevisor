BSP ?= rpi4

TARGET            = aarch64-unknown-none-softfloat
KERNEL_BIN        = kernel8.img
QEMU_BINARY       = qemu-system-aarch64
QEMU_MACHINE_TYPE =
QEMU_RELEASE_ARGS = -d in_asm -display none
OBJDUMP_BINARY    = aarch64-linux-gnu-objdump
NM_BINARY         = aarch64-linux-gnu-nm
SIZE_BINARY         = aarch64-linux-gnu-size
READELF_BINARY    = aarch64-linux-gnu-readelf
LINKER_FILE       = src/linker.ld
RUSTC_MISC_ARGS   = -C target-cpu=cortex-a72

KERNEL_ELF = target/$(TARGET)/release/kernel
RUSTFLAGS          = -C link-arg=-T$(LINKER_FILE) $(RUSTC_MISC_ARGS)
RUSTFLAGS_PEDANTIC = $(RUSTFLAGS) -D warnings

#FEATURES      = --features bsp_$(BSP)
COMPILER_ARGS = --target=$(TARGET) \
    $(FEATURES)                    \
    --release

RUSTC_CMD   = cargo rustc $(COMPILER_ARGS)
DOC_CMD     = cargo doc $(COMPILER_ARGS)
CLIPPY_CMD  = cargo clippy $(COMPILER_ARGS)
CHECK_CMD   = cargo check $(COMPILER_ARGS)
OBJCOPY_CMD = rust-objcopy \
    --strip-all            \
    -O binary

all: $(KERNEL_BIN)

##------------------------------------------------------------------------------
## Build the kernel ELF
##------------------------------------------------------------------------------
$(KERNEL_ELF):
	$(call colorecho, "\nCompiling kernel - $(BSP)")
	@RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(RUSTC_CMD)
	echo $(RUSTFLAGS) $(RUSTC_CMD)

##------------------------------------------------------------------------------
## Build the stripped kernel binary
##------------------------------------------------------------------------------
$(KERNEL_BIN): $(KERNEL_ELF)
	@$(OBJCOPY_CMD) $(KERNEL_ELF) $(KERNEL_BIN)

##------------------------------------------------------------------------------
## Run readelf
##------------------------------------------------------------------------------
readelf: $(KERNEL_ELF)
	$(call colorecho, "\nLaunching readelf")
	$(READELF_BINARY) --headers $(KERNEL_ELF)

##------------------------------------------------------------------------------
## Run objdump
##------------------------------------------------------------------------------
objdump: $(KERNEL_ELF)
	$(call colorecho, "\nLaunching objdump")
	$(OBJDUMP_BINARY) --disassemble --demangle \
                --section .text   \
                --section .rodata \
                --section .got    \
                $(KERNEL_ELF) | rustfilt

#objdump: $(KERNEL_ELF)
#	$(call colorecho, "\nLaunching objdump")
#	$(OBJDUMP_BINARY) -Ct $(KERNEL_ELF)

nm: $(KERNEL_ELF)
	$(NM_BINARY) --demangle --print-size $(KERNEL_ELF) | sort | rustfilt

size: $(KERNEL_ELF)
	$(SIZE_BINARY) $(KERNEL_ELF)


##------------------------------------------------------------------------------
## Clean
##------------------------------------------------------------------------------
clean:
	rm -rf target $(KERNEL_BIN)