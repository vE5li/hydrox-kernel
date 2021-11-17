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
static ALLOCATOR: memory::heap::Allocator = memory::heap::Allocator::new(0x40000, 0x1000);

#[no_mangle]
pub extern fn kernel_main() -> ! {

    peripherals::initialize();

    peripherals::gpio::idle(5000);

    peripherals::logger::log_tmp("kernel initialized\n");

    // temp

    //logp!("foo");

    peripherals::logger::log_tmp("starting blink test\n");

    use peripherals::gpio::*;

    set_function(Pin::V5, Function::Output);
    set_function(Pin::V6, Function::Output);

    peripherals::uart::write_character_blocking('1');

    let boxed = alloc::boxed::Box::new(50u32);
    peripherals::uart::write_character_blocking(*boxed as u8 as char);

    peripherals::uart::write_character_blocking('3');

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
    peripherals::uart::write_character_blocking('1');
    loop {}
}

#[lang = "eh_personality"]
pub extern fn eh_personality() {
    peripherals::uart::write_character_blocking('2');
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    peripherals::uart::write_character_blocking('3');
    loop {}
}
