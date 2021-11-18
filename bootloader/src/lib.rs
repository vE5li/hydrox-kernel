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

use core::alloc::Layout;
use core::panic::PanicInfo;

#[global_allocator]
static ALLOCATOR: memory::heap::Allocator = memory::heap::Allocator::new(0x40000, 0xf000);

#[no_mangle]
pub extern fn kernel_main() -> ! {

    peripherals::initialize();

    success!("kernel initialized");

    // temp

    use peripherals::gpio::*;

    set_function(Pin::V5, Function::Output);
    set_function(Pin::V6, Function::Output);

    let boxed = alloc::boxed::Box::new(50u32);

    log_line!("starting blink test");

    loop {
        set_state(Pin::V5, true);
        set_state(Pin::V6, false);
        idle(2500000);
        set_state(Pin::V5, false);
        set_state(Pin::V6, true);
        idle(2500000);
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
