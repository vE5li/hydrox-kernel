#[macro_use]
pub mod logger;
pub mod gpio;
pub mod uart;
pub mod mailbox;

const PERIPHERALS_BASE: usize = 0xfe000000;

pub fn initialize() {

    uart::initialize();

    success!("peripherals initialized");
}
