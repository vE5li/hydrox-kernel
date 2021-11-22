macro_rules! log {
    ($($arguments:tt)*)   => ({$crate::peripherals::logger::log(format_args!($($arguments)*));});
}

macro_rules! log_line {
    ($format:expr)                      => (log!(concat!("[ kernel ] ", $format, "\n")));
    ($format:expr, $($arguments:tt)*)   => (log!(concat!("[ kernel ] ", $format, "\n"), $($arguments)*));
}

macro_rules! error {
    ($format:expr)                      => (log_line!(concat!("[ error ] ", $format)));
    ($format:expr, $($arguments:tt)*)   => (log_line!(concat!("[ error ] ", $format), $($arguments)*));
}

macro_rules! success {
    ($format:expr)                      => (log_line!(concat!("[ success ] ", $format)));
    ($format:expr, $($arguments:tt)*)   => (log_line!(concat!("[ success ] ", $format), $($arguments)*));
}

use core::fmt::{ Write, Arguments, Result };

struct Logger {
    framebuffer: Option<::graphics::Framebuffer>,
    cursor_x: usize,
    cursor_y: usize,
}

impl Logger {

    pub const fn new() -> Self {
        let framebuffer = None;
        let cursor_x = 0;
        let cursor_y = 0;
        return Self { framebuffer, cursor_x, cursor_y };
    }

    pub fn set_framebuffer(&mut self, framebuffer: ::graphics::Framebuffer) {
        self.framebuffer = Some(framebuffer);
    }
}

impl Write for Logger {

    fn write_str(&mut self, message: &str) -> Result {
        use peripherals::uart::write_character_blocking;

        // use as bytes instead of chars to improve performance
        message.as_bytes().iter().for_each(|byte| {
            write_character_blocking(*byte as char);

            if let Some(framebuffer) = &mut self.framebuffer {
                if *byte as char == '\n' {
                    self.cursor_x = 0;

                    if self.cursor_y == 500 { // total_lines * (FONT_HEIGHT + gap)
                        let line_byte_size = framebuffer.bytes_per_pixel * framebuffer.width * 10; // FONT_HEIGHT + gap
                        let second_line_address = framebuffer.address + line_byte_size;
                        let size = framebuffer.size - line_byte_size * 2;
                        ::memory::memmove(framebuffer.address as *const u8 as *mut u8, second_line_address as *const u8 as *mut u8, size);
                    } else {
                        self.cursor_y += 10; // FONT_HEIGHT + gap
                    }

                } else {
                    framebuffer.draw_character(self.cursor_x, self.cursor_y, *byte as char, 0xAAAAAA, 0x000000);
                    self.cursor_x += 8;
                }
            }
        });

        return Ok(());
    }
}

static mut LOGGER: Logger = Logger::new();

pub fn set_framebuffer(framebuffer: ::graphics::Framebuffer) {
    unsafe { LOGGER.set_framebuffer(framebuffer); }
}

pub fn log(args: Arguments) {
    unsafe { LOGGER.write_fmt(args).unwrap(); }
}
