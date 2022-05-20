#[macro_use]
pub mod register;
#[macro_use]
pub mod logger;
pub mod interrupts;
pub mod timer;
pub mod gpio;
pub mod uart;
pub mod mailbox;

const PERIPHERALS_BASE: usize = 0xfe000000;

pub fn initialize() {

    uart::initialize();

    interrupts::enable_interrupt_controller();

    success!("peripherals initialized");
}
