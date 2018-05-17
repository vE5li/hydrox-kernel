use core::fmt;

// static instance
static mut LOGGER: Logger = Logger {};

// serial/ethernet logger
struct Logger {}

// writing formatted fmt::Arguments
impl fmt::Write for Logger {
    fn write_str(&mut self, message: &str) -> fmt::Result {
        for character in message.chars() {
            unsafe { super::interface::log_character(character as u8); }
        }
        Ok(())
    }
}

// log partial (macro)
macro_rules! logp {
    ($($arg:tt)*) => ({$crate::peripherals::logger::log(format_args!($($arg)*));});
}

// log line (macro)
macro_rules! log {
    ($fmt:expr) => (logp!(concat!("[ kernel ] ", $fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (logp!(concat!("[ kernel ] ", $fmt, "\n"), $($arg)*));
}

// log function called by the macros
pub fn log(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe { LOGGER.write_fmt(args).unwrap(); }
}
