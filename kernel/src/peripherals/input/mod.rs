mod event;

pub use self::event::{ Event, Mode };

// temporary function
pub fn idle() -> ! {
    use super::gpio::{ Pin, set_state };

    // never exit
    loop {
        let event = Event::from_u16(unsafe { super::interface::read_event() });
        match event.tupled() {

            // reboot on ctrl + shift + q
            (Mode::Advanced, b'@')  => super::reboot(),

            // turn off gpio on super + o
            (Mode::Window, b'o')    => set_state(Pin::P40, false),

            // turn on gpio on super + p
            (Mode::Window, b'p')    => set_state(Pin::P40, true),

            // no valid shortcut
            _                       => {
                if let Some(character) = event.ascii() {
                    log!("key pressed {}", character);
                }
            },
        }
    }
}
