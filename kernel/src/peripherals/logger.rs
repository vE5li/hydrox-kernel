// log partial (macro)
macro_rules! logp {
    ($($arguments:tt)*)   => ({$crate::peripherals::logger::log(format_args!($($arguments)*));});
}

// log line (macro)
macro_rules! log {
    ($format:expr)                      => (logp!(concat!("[ kernel ] ", $format, "\n")));
    ($format:expr, $($arguments:tt)*)   => (logp!(concat!("[ kernel ] ", $format, "\n"), $($arguments)*));
}

// log an error message (macro)
macro_rules! error {
    ($format:expr)                      => (log!(concat!("[ error ] ", $format)));
    ($format:expr, $($arguments:tt)*)   => (log!(concat!("[ error ] ", $format), $($arguments)*));
}

// log a success (macro)
macro_rules! success {
    ($format:expr)                      => (log!(concat!("[ success ] ", $format)));
    ($format:expr, $($arguments:tt)*)   => (log!(concat!("[ success ] ", $format), $($arguments)*));
}

use core::fmt;

// serial/ethernet logger
struct Logger {}

// implement fmt::write for the logger
impl fmt::Write for Logger {

    // writing formatted fmt::Arguments
    fn write_str(&mut self, message: &str) -> fmt::Result {
        for character in message.chars() {
            unsafe { super::interface::log_character(character as u8); }
        }
        Ok(())
    }
}

// static instance
static mut LOGGER: Logger = Logger {};

// log function called by the macros
pub fn log(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe { LOGGER.write_fmt(args).unwrap(); }
}
