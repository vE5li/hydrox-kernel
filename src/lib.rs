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
mod graphics;
mod memory;
mod steckhalma;
mod steckhalma_draw;

use alloc::string::String;
use core::alloc::Layout;
use core::panic::PanicInfo;
use peripherals::gpio::*;
use peripherals::mailbox::*;
use peripherals::uart::*;
use steckhalma::*;
use steckhalma_draw::*;

#[global_allocator]
static ALLOCATOR: memory::heap::Allocator = memory::heap::Allocator::new(0x70000, 0x10000);

extern {
    pub fn get_el() -> u64;
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
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

    let mut cursor_pos = Pos { x: 0, y: 0 };

    loop {
        let board = Board::new();

        let draw_settings = DrawSettings {
            colour_background: 0x888888,
            colour_border: 0x3b3b3b,
            colour_nopeg: 0x000000,
            colour_peg: 0x666666,
            colour_highlight: 0xff3333,
            field_size: 60,
            margin: 5,
        };

        draw(&mut framebuffer, &draw_settings, (150, 50), &board, cursor_pos);

        let user_input = read_character_blocking();

        let try_move_cursor = |direction, cursor_pos: &mut Pos| {
            let new_cursor_pos = cursor_pos.shift(direction, 1);
            if new_cursor_pos.in_range() {
                *cursor_pos = new_cursor_pos;
            }
        };

        match user_input {
            'h' => try_move_cursor(Direction::Left, &mut cursor_pos),
            'j' => try_move_cursor(Direction::Down, &mut cursor_pos),
            'k' => try_move_cursor(Direction::Up, &mut cursor_pos),
            'l' => try_move_cursor(Direction::Right, &mut cursor_pos),
            _invalid => {},
        }
    }
}

#[no_mangle]
#[lang = "oom"]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn oom(_layout: Layout) -> ! {
    error!("out of memory");
    loop {}
}

#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {
    error!("unwind");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // location.caller

    if let Some(location) = info.location() {
        match info.message() {
            Some(message) => log_line!(
                "[ panic ] {} {}:{}: {}",
                location.file(),
                location.line(),
                location.column(),
                message
            ),
            None => log_line!(
                "[ panic ] {} {}:{}",
                location.file(),
                location.line(),
                location.column()
            ),
        }
    } else {
        match info.message() {
            Some(message) => log_line!("[ panic ] {}", message),
            None => log_line!("[ panic ]"),
        }
    }

    loop {}
}
