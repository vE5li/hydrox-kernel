#[macro_use]
pub mod logger;
pub mod gpio;
pub mod uart;

const PERIPHERALS_BASE: usize = 0xfe000000;

pub fn initialize() {

    uart::initialize();

    // turn off the act led
    //let mut letter = Letter::new();
    //letter.clear_tags();
    //letter.push_tag(MailboxTag::SetPowerState, &[130, 0]);
    //letter.push_end_tag();
    //letter.send(Channel::Tags);
    //letter.receive(Channel::Tags);

    //set_function(Pin::V5, Function::Output);
    //set_function(Pin::V6, Function::Output);

    success!("peripherals initialized");
}
