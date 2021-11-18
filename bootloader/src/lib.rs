#![feature(allocator_api)]
#![feature(lang_items)]
#![feature(asm)]
#![no_builtins]
#![no_main]
#![no_std]

extern crate alloc;

#[macro_use]
mod peripherals;
mod memory;
mod graphics;

use core::alloc::Layout;
use core::panic::PanicInfo;
use peripherals::uart::*;
use peripherals::gpio::*;

#[global_allocator]
static ALLOCATOR: memory::heap::Allocator = memory::heap::Allocator::new(0x60000, 0x20000);

#[no_mangle]
pub extern fn kernel_main() -> ! {

    peripherals::initialize();

    let mut framebuffer = graphics::initialize();

    success!("kernel initialized");

    // gpio test

    log_line!("starting gpio test");

    set_function(Pin::V5, Function::Output);
    set_function(Pin::V6, Function::Output);

    set_state(Pin::V5, true);
    set_state(Pin::V6, false);

    // graphics test

    log_line!("starting graphics test");

    framebuffer.draw_rectangle(0, 0, 30, 30, 0xAAAAAA, 0xAAAAAA);
    framebuffer.draw_rectangle(40, 0, 30, 30, 0xAA5500, 0xAA5500);
    framebuffer.draw_rectangle(80, 0, 30, 30, 0xAA0000, 0x0000AA);

    // heap allocation test

    log_line!("starting allocation test");

    let boxed = alloc::boxed::Box::new(50u32);

    if *boxed != 50 {
        panic!("incorrect box value");
    }

    success!("allocation test passed");

    // echo test

    log_line!("starting echo test");

    loop {
        let character = read_character_blocking();
        write_character_blocking(character);
    }
}

#[no_mangle]
#[lang = "oom"]
pub extern fn oom(_layout: Layout) -> ! {
    error!("out of memory");
    loop {}
}

#[lang = "eh_personality"]
pub extern fn eh_personality() {
    error!("unwind");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    error!("kernel panic");
    loop {}
}
