# files
OUTPUT ?= /usr/bin/loader

# rust features
FEAT ?= input

# targets
RTARGET := target/debug/alb1-loader

# build
all: build

# nothing
clean:
	cargo clean

# compile
build:
	cargo build --features "$(FEAT)"

# verbose feature
verbose:
	cargo build --features "$(FEAT) verbose"

# build without features
minimal:
	cargo build

# copy
copy:
	cp $(RTARGET) $(OUTPUT)

# run
run:
	cargo run
