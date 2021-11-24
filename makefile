# directories
LINKER_DIR ?= linker
BINARY_DIR ?= binary
SOURCE_DIR ?= src
IMAGE ?= raspios.img

# toolchain
TOOLCHAIN ?= aarch64-none-elf-

# mount points
MOUNT_POINT ?= /mnt
SD_CARD ?= /dev/sda

# output files
KERNEL_BINARY ?= $(BINARY_DIR)/kernel.bin
KERNEL_ELF := $(LINKER_DIR)/kernel.elf

# files
CLEAN_FILES := bootcode.bin config.txt fixup4.dat kernel.bin start4.elf
OBJECT_FILES := start.s.o
CONFIGURATION_FILE := $(MOUNT_POINT)/config.txt

# xargo
LINKER_FLAGS ?= -lgcc -nostdlib -Wl,--build-id=none
RUST_TARGET ?= aarch64-feo-kernel
RUST_OBJECT := target/$(RUST_TARGET)/release/libfeo_kernel.a

# build
all: build link

# remove object files
clean:
	cargo clean
	rm $(KERNEL_ELF)
	rm $(addprefix $(LINKER_DIR)/, $(OBJECT_FILES))

# assemble the source code
%.s.o: $(SOURCE_DIR)/%.s
	$(TOOLCHAIN)as -o $(LINKER_DIR)/$@ $<

# compile the master rust code with almost all features
build:
	RUST_TARGET_PATH=$(shell pwd) xargo build --release --target=$(RUST_TARGET)

# link the rust and the assembly with gcc
link: $(OBJECT_FILES)
	$(TOOLCHAIN)gcc -o $(KERNEL_ELF) -T $(LINKER_DIR)/linker.ld $(addprefix $(LINKER_DIR)/, $^) $(RUST_OBJECT) $(LINKER_FLAGS)
	$(TOOLCHAIN)objcopy $(KERNEL_ELF) -O binary $(KERNEL_BINARY)

# mount the sd-card
mount:
	mountpoint -q $(MOUNT_POINT) || mount $(SD_CARD)1 $(MOUNT_POINT)
	@ls $(MOUNT_POINT)

# unmount the sd-card
unmount:
	mountpoint -q $(MOUNT_POINT) && umount $(MOUNT_POINT) || :

# install a blank raspbian image
zero: unmount
	dd bs=4M conv=fsync if=$(IMAGE) of=$(SD_CARD) || :

# copy the kernel.bin to the sd-card
copy: mount
	cp -f $(KERNEL_BINARY) $(MOUNT_POINT)/

# overwrite the config file
config: copy
	echo "arm_64bit=1" > $(CONFIGURATION_FILE)
	echo "kernel=kernel.bin" >> $(CONFIGURATION_FILE)
	echo "core_freq_min=500" >> $(CONFIGURATION_FILE)
	echo "dtoverlay=disable-bt" >> $(CONFIGURATION_FILE)
	echo "hdmi_group=2" >> $(CONFIGURATION_FILE)
	echo "hdmi_mode=9" >> $(CONFIGURATION_FILE)

# make a workiing
export: zero config

# show disassembled code
opcode: build
	$(TOOLCHAIN)objdump -d $(KERNEL_ELF)
