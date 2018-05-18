// raspberry pi 3 peripherals base
const PERIPHERALS_BASE: usize       = 0x3f000000;

// get the address of a peripheral based on it's offet so the peripherals base
macro_rules! peripheral {
    (gpio) => ($crate::peripherals::PERIPHERALS_BASE + 0x200000);
    (mailbox0) => ($crate::peripherals::PERIPHERALS_BASE + 0xb880);
    (mailbox1) => ($crate::peripherals::PERIPHERALS_BASE + 0xb8a0);
    (undefined) => (panic!("undefined peripheral {}", undefined));
}

pub mod interface;
#[macro_use]
pub mod logger;
pub mod input;
pub mod mailman;
pub mod gpio;

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
    gpio::set_function(gpio::Pin::V21, gpio::Function::Output);
}

// reboot the device
pub fn reboot() -> ! {
    log!("reboot!");
    loop {}
}

// temporary function
pub fn idle() -> ! {
    use self::input::{ InputEvent, InputMode };

    // loop endlessly
    loop {
        let event = InputEvent::from_u16(unsafe { interface::read_event() });
        match event.tupled() {

            // reboot on ctrl + shift + q
            (InputMode::Advanced, b'@') => reboot(),

            // turn off gpio on super + o
            (InputMode::Window, b'o') => self::gpio::set_state(gpio::Pin::V21, false),

            // turn on gpio on super + p
            (InputMode::Window, b'p') => self::gpio::set_state(gpio::Pin::V21, true),

            // no valid shortcut
            _ => {
                if let Some(character) = event.ascii() {
                    log!("key pressed {}", character);
                }
            },
        }
    }
}
