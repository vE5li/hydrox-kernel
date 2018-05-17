# files
OUTPUT ?= /usr/bin/loader

# targets
RTARGET := target/debug/alb1-loader

# build
all: build

# nothing
clean:
	cargo clean

# compile
build:
	cargo build --features input

# verbose build for configuring
verbose:
	cargo build --features "input verbose"

# compile without input system
minimal:
	cargo build

# copy
copy:
	cp $(RTARGET) $(OUTPUT)

# run
run:
	cargo run
