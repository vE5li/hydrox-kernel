mod event;

pub use self::event::{ Event, Mode };

// temporary function
pub fn idle() -> ! {
    use super::gpio::{ Pin, set_state, pulse };

    // never exit
    loop {
        let event = Event::from_u16(unsafe { super::interface::read_event() });
        match event.tupled() {

            // reboot on ctrl + shift + q
            (Mode::Advanced, b'@')  => super::reboot(),

            // turn on gpio on super + p
            (Mode::Window, b'p')    => set_state(Pin::P40, true),

            // turn off gpio on super + o
            (Mode::Window, b'o')    => set_state(Pin::P40, false),

            // pulse gpio on super + i
            (Mode::Window, b'i')    => pulse(Pin::P11),

            // pulse gpio on super + u
            (Mode::Window, b'u')    => pulse(Pin::P12),

            // pulse gpio on super + z
            (Mode::Window, b'z')    => pulse(Pin::P13),

            // pulse gpio on super + t
            (Mode::Window, b't')    => pulse(Pin::P15),

            // pulse gpio on super + r
            (Mode::Window, b'r')    => pulse(Pin::P16),

            // pulse gpio on super + e
            (Mode::Window, b'e')    => pulse(Pin::P18),

            // pulse gpio on super + w
            (Mode::Window, b'w')    => pulse(Pin::P29),

            // pulse gpio on super + q
            (Mode::Window, b'q')    => pulse(Pin::P31),

            // not bound
            _                       => continue,
        }
    }
}
