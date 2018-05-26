pub mod interface;
#[macro_use]
pub mod logger;
pub mod mailman;
pub mod gpio;
pub mod input;

// raspberry pi 3 peripherals base
const PERIPHERALS_BASE: usize       = 0x3f000000;

// initialize all peripherals
pub fn initialize() {
    use self::mailman::{ Letter, Channel, MailboxTag };
    use self::gpio::{ Pin, Function, set_function };

    success!("kernel started booting");

    // turn off the act led
    let mut letter = Letter::new();
    letter.clear_tags();
    letter.push_tag(MailboxTag::SetPowerState, &[130, 0]);
    letter.push_end_tag();
    letter.send(Channel::Tags);
    letter.receive(Channel::Tags);

    // set physical pin 40 as output
    set_function(Pin::P40, Function::Output);
    set_function(Pin::P11, Function::Output);
    set_function(Pin::P12, Function::Output);
    set_function(Pin::P13, Function::Output);
    set_function(Pin::P15, Function::Output);
    set_function(Pin::P16, Function::Output);
    set_function(Pin::P18, Function::Output);
    set_function(Pin::P29, Function::Output);
    set_function(Pin::P31, Function::Output);

    success!("peripherals initialized");
}

// reboot the device
pub fn reboot() -> ! {
    success!("rebooting");
    loop {}
}
