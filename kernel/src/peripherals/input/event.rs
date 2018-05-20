// event operation mode
#[derive(Copy, Clone)]
pub enum Mode {
    None,
    Action,
    Capital,
    Setting,
    Window,
    Advanced,
    Undefined(u8),
}

// user input event
#[derive(Copy, Clone)]
pub struct Event {
    modifiers:  u8,
    code:       u8,
}

// implement input event
impl Event {

    // get an event structure from a raw u16
    pub fn from_u16(raw: u16) -> Event {
        Event {
            modifiers:  (raw >> 8) as u8,
            code:       raw as u8,
        }
    }

    // get the operation mode
    pub fn mode(&self) -> Mode {
        match self.modifiers {
            0b00000000  => Mode::None,
            0b00000001  => Mode::Action,
            0b00000010  => Mode::Capital,
            0b00000100  => Mode::Setting,
            0b00001000  => Mode::Window,
            0b00000110  => Mode::Advanced,
            modifiers   => Mode::Undefined(modifiers),
        }
    }

    // get a tuple for matching
    pub fn tupled(&self) -> (Mode, u8) {
        (self.mode(), self.code)
    }

    // attempt to create an ascii character from the input event
    pub fn ascii(&self) -> Option<char> {
        match self.code {
            32...126    => Some(self.code as char),
            _           => None,
        }
    }
}
