// get the address of a peripheral based on it's offet so the peripherals base
macro_rules! peripheral {
    (gpio)          => ($crate::peripherals::PERIPHERALS_BASE + 0x200000);
    (mailbox0)      => ($crate::peripherals::PERIPHERALS_BASE + 0xb880);
    (mailbox1)      => ($crate::peripherals::PERIPHERALS_BASE + 0xb8a0);
}

// raspberry pi 3 peripherals base
const PERIPHERALS_BASE: usize       = 0x3f000000;

pub mod interface;
#[macro_use]
pub mod logger;
pub mod mailman;
pub mod gpio;
pub mod input;

// initialize all peripherals
pub fn initialize() {

    // turn off the act led
    let mut letter = mailman::Letter::new();
    letter.clear_tags();
    letter.push_tag(mailman::MailboxTag::SetPowerState, &[130, 0]);
    letter.push_end_tag();
    letter.send(mailman::Channel::Tags);
    letter.receive(mailman::Channel::Tags);

    // set physical pin 40 as output
    gpio::set_function(gpio::Pin::P40, gpio::Function::Output);
}

// reboot the device
pub fn reboot() -> ! {
    log!("reboot!");
    loop {}
}
