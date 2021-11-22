# directories
LINKER_DIR ?= linker
BINARY_DIR ?= binary
SOURCE_DIR ?= src
IMAGE ?= raspios.img

# compiler
CROSS ?= aarch64-none-elf-
#LASM_FLAGS ?= -nostdlib

# mount points
MOUNT_POINT ?= /mnt
DEVICE ?= /dev/sda

# output files
OUTPUT ?= $(BINARY_DIR)/kernel.bin

# files
CLEAN_FILES := overlays start_*.elf kernel7.img kernel.img issue.txt COPYING.linux LICENCE.broadcom LICENSE.oracle *.dtb fixup_cd.dat fixup_db.dat fixup_x.dat
SOURCE_FILES := start.s.o
#UNITY := $(LINKER_DIR)/unity.s.o
CONFIG := $(MOUNT_POINT)/config.txt

# xargo
LFLAGS ?= -lgcc -nostdlib -Wl,--build-id=none
RUST_TARGET ?= aarch64-alb1
RUST_OUTPUT := target/$(RUST_TARGET)/release/libalb1.a
KERNEL_ELF := $(LINKER_DIR)/kernel.elf

# build
all: build link

# remove the object files
clean:
	#rm $(UNITY)

# assemble the source code
%.s.o: $(SOURCE_DIR)/%.s
	$(CROSS)as -o $(LINKER_DIR)/$@ $<

# compile the master rust code with almost all features
build:
	RUST_TARGET_PATH=$(shell pwd) xargo build --release --target=$(RUST_TARGET)

# link the rust and the assembly with gcc
link: $(SOURCE_FILES)
	$(CROSS)gcc -o $(KERNEL_ELF) -T $(LINKER_DIR)/linker.ld $(addprefix $(LINKER_DIR)/, $^) $(RUST_OUTPUT) $(LFLAGS)
	$(CROSS)objcopy $(KERNEL_ELF) -O binary $(OUTPUT)

# mount the sd-card
mount:
	mountpoint -q $(MOUNT_POINT) || mount $(DEVICE)1 $(MOUNT_POINT)
	@ls $(MOUNT_POINT)

# unmount the sd-card
unmount:
	mountpoint -q $(MOUNT_POINT) && umount $(MOUNT_POINT) || :

# install a blank raspbian image
zero: unmount
	dd bs=4M conv=fsync if=$(IMAGE) of=$(DEVICE) || :

# copy the kernel.bin to the sd-card
copy: mount
	cp -f $(OUTPUT) $(MOUNT_POINT)/

# arm_boost=1 ?
# overwrite the config file
config: copy
	echo "arm_64bit=1" > $(CONFIG)
	echo "kernel=kernel.bin" >> $(CONFIG)
	echo "disable_commandline_tags=1" >> $(CONFIG)
	echo "core_freq_min=500" >> $(CONFIG)
	echo "dtoverlay=disable-bt=500" >> $(CONFIG)
	rm -rf $(addprefix $(MOUNT_POINT)/, $(CLEAN_FILES))

# make a workiing
export: zero config

# show disassembled code
opcode: build
	$(CROSS)objdump -d $(KERNEL_ELF)
