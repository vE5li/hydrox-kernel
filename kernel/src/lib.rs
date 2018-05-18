#![feature(exclusive_range_pattern)]
#![feature(lang_items)]
#![no_builtins]
#![no_std]

// mudules
#[macro_use]
mod memory;
#[macro_use]
mod peripherals;
#[cfg(feature = "graphics")]
mod graphics;

// kernel main
#[no_mangle]
pub extern fn kernel_main() -> ! {

    // wait for the loader to be able to receive data again
    for _ in 0..1000 {}

    // initialize hardware
    peripherals::initialize();

    // initialize graphics is included
    #[cfg(feature = "graphics")]
    graphics::initialize();

    // kernel booted successfully
    log!("started successfully");

    // temporary user input handler
    peripherals::idle();
}

// kernel panic
#[no_mangle]
#[lang = "panic_fmt"]
pub extern fn panic(message: core::fmt::Arguments, file: &'static str, line: u32, column: u32) -> ! {
    log!(" [ panic ] paniced in file {} at line {} : {}", file, line, column);
    log!("{}", message);
    loop{}
}
