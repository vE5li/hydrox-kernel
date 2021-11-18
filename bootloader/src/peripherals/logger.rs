macro_rules! log {
    ($($arguments:tt)*)   => ({$crate::peripherals::logger::log(format_args!($($arguments)*));});
}

macro_rules! log_line {
    ($format:expr)                      => (log!(concat!("[ kernel ] ", $format, "\n\r")));
    ($format:expr, $($arguments:tt)*)   => (log!(concat!("[ kernel ] ", $format, "\n\r"), $($arguments)*));
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

struct Logger {}

impl Write for Logger {

    fn write_fmt(mut self: &mut Self, args: Arguments<'_>) -> Result {
        self.write_str(args.as_str().unwrap()); // TEMP
        return Ok(());
    }

    fn write_str(&mut self, message: &str) -> Result {
        use peripherals::uart::write_character_blocking;
        message.chars().for_each(|character| write_character_blocking(character));
        return Ok(());
    }
}

static mut LOGGER: Logger = Logger {};

pub fn log(args: Arguments) {
    unsafe { LOGGER.write_fmt(args).unwrap(); }
}

pub fn log_hex(number: usize) {
    use peripherals::uart::write_character_blocking;

    write_character_blocking('0');
    write_character_blocking('x');

    for index in (0..16).rev() {
        let byte = (number >> (index * 4)) & 0b1111;
        let character = match byte {
            0..=9 => write_character_blocking((48 + byte) as u8 as char),
            _bigger => write_character_blocking((87 + byte) as u8 as char),
        };
    }
}
