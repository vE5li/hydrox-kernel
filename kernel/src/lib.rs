#![feature(global_allocator, allocator_api, alloc)]
#![feature(exclusive_range_pattern)]
#![feature(lang_items)]
#![feature(const_fn)]
#![no_builtins]
#![no_std]

// alloc and collection
#[macro_use]
extern crate alloc;

// mudules
#[macro_use]
mod memory;
#[macro_use]
mod peripherals;
#[cfg(feature = "graphics")]
mod graphics;

// reexport mem functions to preserve the symbols
pub use memory::{ memcpy, memmove, memset, memcmp };

// global heap allocator
#[global_allocator]
static ALLOCATOR: memory::heap::Allocator = memory::heap::Allocator::new(0x6000, 0x1000);

// kernel main
#[no_mangle]
pub extern fn kernel_main() -> ! {

    // initialize hardware
    peripherals::initialize();

    // initialize graphics
    #[cfg(feature = "graphics")]
    graphics::initialize();

    success!("kernel started");

    // temporary user input handler
    peripherals::input::idle();
}

// kernel panic
#[no_mangle]
#[lang = "panic_fmt"]
pub extern fn panic(message: core::fmt::Arguments, file: &'static str, line: u32, column: u32) -> ! {
    log!(" [ panic ] fatal error; file {}; line {}; column {}", file, line, column);
    log!("{}", message);
    loop {}
}

// out of memory
#[no_mangle]
#[lang = "oom"]
pub extern fn oom() -> ! {
    panic!("out of memory");
}
