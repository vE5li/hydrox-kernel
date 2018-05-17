# directories
LINKER_DIR ?= linker
BINARY_DIR ?= binary
SOURCE_DIR ?= src
IMAGE ?= /image/raspbian.img

# compiler
CROSS ?= aarch64-elf-
LFLAGS ?= -nostdlib

# targets
TARGET ?= /card
STARGET ?= /dev/serial0
DEVICE ?= /dev/sdb

# output files
LOADER_ELF ?= $(LINKER_DIR)/bootloader.elf
OUTPUT ?= $(BINARY_DIR)/bootloader.bin

# files
CLEAN_FILES := overlays start_*.elf kernel7.img kernel.img issue.txt COPYING.linux LICENCE.broadcom LICENSE.oracle *.dtb fixup_cd.dat fixup_db.dat fixup_x.dat
SOURCE_FILES := macros.s peripherals.s string.s mailman.s serial.s ethernet.s start.s
UNITY := $(LINKER_DIR)/unity.s.o
CONFIG := $(TARGET)/config.txt

# build
all: build

# remove the object files
clean:
	rm $(UNITY)

# build the bootloader
build: link
	$(CROSS)objcopy $(LOADER_ELF) -O binary $(OUTPUT)

# assemble the source files into one object file
assemble: $(addprefix $(SOURCE_DIR)/, $(SOURCE_FILES))
	$(CROSS)as -o $(UNITY) $^

# link the object file
link: assemble
	$(CROSS)ld -o $(LOADER_ELF) -T $(LINKER_DIR)/linker.ld $(UNITY) $(LFLAGS)

# mount the sd-card
mount:
	mountpoint -q $(TARGET) || mount $(DEVICE)1 $(TARGET)
	@ls $(TARGET)

# unmount the sd-card
unmount:
	mountpoint -q $(TARGET) && umount $(TARGET) || :

# install a blank raspbian image
zero: unmount
	dd bs=4M conv=fsync if=$(IMAGE) of=$(DEVICE) || :

# copy the bootloader.bin to the sd-card
copy: build mount
	cp -f $(OUTPUT) $(TARGET)/

# overwrite the config file
config: copy
	echo "kernel_old=1" > $(CONFIG)
	echo "disable_splash=1" >> $(CONFIG)
	echo "kernel=bootloader.bin" >> $(CONFIG)
	echo "arm_control=0x200" >> $(CONFIG)
	echo "disable_commandline_tags=1" >> $(CONFIG)
	echo "disable_overscan=1" >> $(CONFIG)
	echo "framebuffer_swap=0" >> $(CONFIG)
	echo "enable_uart=1" >> $(CONFIG)
	echo "force_turbo=1" >> $(CONFIG)
	echo "boot_delay=1" >> $(CONFIG)
	rm -rf $(addprefix $(TARGET)/, $(CLEAN_FILES))

# make a workiing
export: zero config

# show disassembled code
opcode: build
	$(CROSS)objdump -d $(LOADER_ELF)
