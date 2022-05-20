#![feature(allocator_api)]
#![feature(panic_info_message)]
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
use peripherals::timer::*;
use peripherals::uart::*;
use peripherals::gpio::*;
use peripherals::mailbox::*;
use alloc::string::String;

#[global_allocator]
static ALLOCATOR: memory::heap::Allocator = memory::heap::Allocator::new(0x70000, 0x10000);

extern {
    pub fn get_el() -> u64;
    pub fn irq_init_vectors();
    pub fn irq_enable();
}

#[no_mangle]
pub extern fn kernel_main() -> ! {

    peripherals::initialize();

    let mut framebuffer = graphics::initialize();

    success!("kernel initialized");
    log_line!("elevation level {}", unsafe { get_el() });

    let mut message = Message::<30>::new();
    message.get_firmware_version_request();
    message.get_board_model_request();
    message.get_board_revision_request();
    message.get_board_serial_request();
    message.get_arm_memory_request();
    message.get_video_core_memory_request();
    message.finalize_send_receive(Channel::Tags);

    let firmware_version = message.get_firmware_version_response();
    let board_model = message.get_board_model_response();
    let board_revision = message.get_board_revision_response();
    let board_serial = message.get_board_serial_response();
    let arm_memory_layout = message.get_arm_memory_response();
    let video_core_memory_layout = message.get_video_core_memory_response();

    log_line!("[ device ] firmware version: 0x{:x}", firmware_version);
    log_line!("[ device ] board model: 0x{:x}", board_model);
    log_line!("[ device ] board revision: 0x{:x}", board_revision);
    log_line!("[ device ] board serial: 0x{:x}", board_serial);
    log_line!("[ device ] arm memory: {}", arm_memory_layout);
    log_line!("[ device ] video core memory: {}", video_core_memory_layout);

    log_line!("turning on status led");

    let mut message = Message::<20>::new();
    message.set_led_status_request(OnBoardLed::Status, true);
    message.set_led_status_request(OnBoardLed::Power, false);
    message.finalize_send_receive(Channel::Tags);

    log_line!("fetch device temperature");

    // gpio test

    log_line!("starting gpio test");

    set_function(Pin::Virtual5, Function::Output);
    set_function(Pin::Virtual6, Function::Output);

    set_state(Pin::Virtual5, true);
    set_state(Pin::Virtual6, false);

    // graphics test

    log_line!("starting graphics test");

    framebuffer.draw_rectangle(600, 0, 30, 30, 0xff0000, 0x00ff00);
    framebuffer.draw_text(600, 40, "i am rectangular", 0x00FF00, 0x000000);

    // heap allocation test

    log_line!("starting allocation test");

    let boxed = alloc::boxed::Box::new(50);
    assert!(*boxed == 50, "incorrect value in box");

    let heap_string = String::from("i live on the heap");
    success!("{}", heap_string);

    success!("allocation test passed");

    // interrupts

    log_line!("enabling interrupts");

    unsafe {
        irq_init_vectors();
        irq_enable();
    }

    // timers

    log_line!("setting system timer 1 and 3");

    set_timer(Timer::Timer1, 2);
    set_timer(Timer::Timer3, 4);

    // echo test

    log_line!("starting echo test");

    loop {

        let mut message = Message::<20>::new();
        message.get_temperature_request();
        message.finalize_send_receive(Channel::Tags);

        let temperature = message.get_temperature_response();
        log_line!("[ device ] temperature: {}C", temperature / 1000);

        let character = read_character_blocking();
        write_character_blocking(character);
    }
}

#[no_mangle]
#[lang = "oom"]
#[allow(improper_ctypes_definitions)]
pub extern fn oom(_layout: Layout) -> ! {
    error!("out of memory");
    loop {}
}

#[lang = "eh_personality"]
pub extern fn eh_personality() {
    error!("unwind");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // location.caller

    if let Some(location) = info.location() {
        match info.message() {
            Some(message) => log_line!("[ panic ] {} {}:{}: {}", location.file(), location.line(), location.column(), message),
            None => log_line!("[ panic ] {} {}:{}", location.file(), location.line(), location.column()),
        }
    } else {
        match info.message() {
            Some(message) => log_line!("[ panic ] {}", message),
            None => log_line!("[ panic ]"),
        }
    }

    loop {}
}
