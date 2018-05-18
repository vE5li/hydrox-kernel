// event operation mode
#[derive(Copy, Clone)]
pub enum InputMode {
    None,
    Action,
    Capital,
    Setting,
    Window,
    Advanced,
    Undefined(u8),
}

// key press event from the user
#[derive(Copy, Clone)]
pub struct InputEvent {
    pub modifiers:  u8,
    pub code:       u8,
}

// implement key event
impl InputEvent {

    // get an event structure from a raw u16
    pub fn from_raw(raw: u16) -> InputEvent {
        InputEvent {
            modifiers:  (raw >> 8) as u8,
            code:       raw as u8,
        }
    }

    // get the operation mode
    pub fn mode(&self) -> InputMode {
        match self.modifiers {
            0b00000000 => InputMode::None,
            0b00000001 => InputMode::Action,
            0b00000010 => InputMode::Capital,
            0b00000100 => InputMode::Setting,
            0b00001000 => InputMode::Window,
            0b00000110 => InputMode::Advanced,
            modifiers => InputMode::Undefined(modifiers),
        }
    }

    // get a tuple for matching
    pub fn tupled(&self) -> (InputMode, u8) {
        (self.mode(), self.code)
    }

    // attempt to create an ascii character from the input event
    pub fn ascii(&self) -> Option<char> {
        match self.code {
            32...126 => Some(self.code as char),
            _ => None,
        }
    }
}
