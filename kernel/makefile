# directories
LINKER_DIR ?= linker
SOURCE_DIR ?= src
OUTPUT_DIR ?= binary

# targets
STARGET ?= /dev/ttyUSB0
RTARGET ?= aarch64-alb1

# loader
ECONFIG ?= ethernet.load
SCONFIG ?= serial.load

# compiler
ED ?= master
LFLAGS ?= -lgcc -nostdlib -Wl,--build-id=none
OUTPUT ?= $(OUTPUT_DIR)/kernel_$(ED).bin
CROSS ?= aarch64-elf-

# files
KERNEL_ELF := $(LINKER_DIR)/kernel.elf
ROUTPUT := target/$(RTARGET)/debug/libalb1_kernel.a
CLEAN_FILES := $(LINKER_DIR)/kernel.* $(LINKER_DIR)/*.s.o
SOURCE_FILES := start.s.o

# xargo
XARGO := RUST_TARGET_PATH=$(shell pwd) xargo build --target=$(RTARGET)

# build
all: build

# remove object files and clean cargo
clean:
	rm -f $(CLEAN_FILES)
	cargo clean

# build the kernel
build: build_$(ED) link

# assemble the source code
%.s.o: $(SOURCE_DIR)/%.s
	$(CROSS)as -o $(LINKER_DIR)/$@ $<

# link the rust and the assembly with gcc
link: $(SOURCE_FILES)
	$(CROSS)gcc -o $(KERNEL_ELF) -T $(LINKER_DIR)/linker.ld $(addprefix $(LINKER_DIR)/, $^) $(ROUTPUT) $(LFLAGS)
	$(CROSS)objcopy $(KERNEL_ELF) -O binary $(OUTPUT)

# compile the master rust code with almost all features
build_master:
	$(XARGO) --features graphics

# compile the headless rust code
build_headless:
	$(XARGO)

# list all sections inside pre linking
sections:
	$(CROSS)objdump -h $(ROUTPUT)
	size -A -d $(ROUTPUT)
	$(CROSS)objdump -h $(LINKER_DIR)/*.s.o
	size -A -d $(LINKER_DIR)/*.s.o
	$(CROSS)objdump -h $(KERNEL_ELF)
	size -A -d $(KERNEL_ELF)

# show disassembled code
opcode: build
	$(CROSS)objdump -d $(KERNEL_ELF)

# open a serial terminal for interfacing with the bootloader and load the kernel
screen:
	@screen $(STARGET) 115200
	@stty -echo
	@loader -c $(SCONFIG) -i
	@stty echo
	@screen $(STARGET) 115200

# start the loader in serial mode
serial:
	@stty -echo
	@loader -c $(SCONFIG)
	@stty echo

# start the loader in ethernet mode
ethernet:
	@stty -echo
	@loader -c $(ECONFIG)
	@stty echo

# recompile every edition
rebuild: clean
	make ED=master
	make ED=headless
