macro_rules! logp {
    ($($arguments:tt)*)   => ({$crate::peripherals::logger::log(format_args!($($arguments)*));});
}

macro_rules! log {
    ($format:expr)                      => (logp!(concat!("[ kernel ] ", $format, "\n")));
    ($format:expr, $($arguments:tt)*)   => (logp!(concat!("[ kernel ] ", $format, "\n"), $($arguments)*));
}

macro_rules! error {
    ($format:expr)                      => (log!(concat!("[ error ] ", $format)));
    ($format:expr, $($arguments:tt)*)   => (log!(concat!("[ error ] ", $format), $($arguments)*));
}

macro_rules! success {
    ($format:expr)                      => (log!(concat!("[ success ] ", $format)));
    ($format:expr, $($arguments:tt)*)   => (log!(concat!("[ success ] ", $format), $($arguments)*));
}

use core::fmt::{ Write, Arguments, Result };

struct Logger {}

impl Write for Logger {

    fn write_str(&mut self, message: &str) -> Result {
        use peripherals::uart::write_character_blocking;
        //for character in message.chars() {
        //    write_character_blocking(character);
        //}
        message.chars().for_each(|character| write_character_blocking(character));
        Ok(())
    }
}

static mut LOGGER: Logger = Logger {};

pub fn log(args: Arguments) {
    unsafe { LOGGER.write_fmt(args).unwrap(); }
}

pub fn log_tmp(message: &str) {
    use peripherals::uart::write_character_blocking;
    message.chars().for_each(|character| write_character_blocking(character));
}
