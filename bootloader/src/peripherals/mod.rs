#[macro_use]
pub mod logger;
pub mod gpio;
pub mod uart;
pub mod mailbox;

const PERIPHERALS_BASE: usize = 0xfe000000;

pub fn initialize() {

    uart::initialize();

    log_line!("turning on status led");

    use self::mailbox::*;

    let mut message = Message::<20>::new();
    message.clear_tags();
    message.push_tag(MailboxTag::SetLEDStatus, &[42, 1]); // 42 = status
    message.push_tag(MailboxTag::SetLEDStatus, &[130, 1]); // 130 = power (state is inverted)
    message.push_end_tag();
    message.send(Channel::Tags);
    message.receive(Channel::Tags);

    success!("peripherals initialized");
}
