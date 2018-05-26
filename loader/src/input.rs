pub use std::thread;
use std::fs::File;
use std::io::Read;

// possible linux driver key states
#[derive(Copy, Clone, Debug)]
enum KeyState {
    Release,
    Press,
    Repeat,
}

// implement key state
impl KeyState {

    // key state from u8
    pub fn from_u8(value: u8) -> Option<KeyState> {
        match value {
            0   => Some(KeyState::Release),
            1   => Some(KeyState::Press),
            2   => Some(KeyState::Repeat),
            _   => None,
        }
    }
}

// modifier keys on the keyboard
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum Modifier {
    None            = 0b00000000,
    Capslock        = 0b00000001,
    Shift           = 0b00000010,
    Control         = 0b00000100,
    Super           = 0b00001000,
    Alternative     = 0b00010000,
    Function        = 0b00100000,
}

// implement modifier
impl Modifier {

    // get a modifer from a string
    pub fn from_slice(source: &str) -> Modifier {
        match source {
            "capslock"      => Modifier::Capslock,
            "shift"         => Modifier::Shift,
            "control"       => Modifier::Control,
            "super"         => Modifier::Super,
            "alternative"   => Modifier::Alternative,
            "function"      => Modifier::Function,
            name            => panic!("[ input ] no modifier key named '{}'", name),
        }
    }
}

// keys actions on event
#[derive(Copy, Clone, Debug)]
enum Action {
    Press(Option<u8>, Option<u8>, Option<u8>),
    Toggle(Modifier),
    Set(Modifier),
    Drop,
}

// input event types
enum InputEvent {
    Keyboard,
}

// implement input event
impl InputEvent {

    // input type from u8
    pub fn from_u8(value: u8) -> Option<InputEvent> {
        match value {
            1   => Some(InputEvent::Keyboard),
            _   => None,
        }
    }
}

// keyboard input handler
pub struct InputHandler {
    translation_table:  [Action; 128],
    input_device:       File,
    modifiers:          u8,
}

// implement input handler
impl InputHandler {

    // new input handler from device file path
    pub fn new(translation_path: &str) -> InputHandler {
        use std::io::{BufRead, BufReader};

        // keyboard device file
        let mut input_device_path: Option<String> = None;
        let mut translation_table = [Action::Drop; 128];

        // attempt to open the translation file
        let mut keycode: u8 = 0;
        let reader = BufReader::new(File::open(translation_path).expect(&format!("[ loader ] unable to open specified translation file '{}'", translation_path)));

        // loop through every line and parse it unless it's commented
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.len() == 0 || line.chars().nth(0).unwrap() == '#' {
                    continue;
                }

                // get a "stack" of words from the line and pop all values
                let mut words: Vec<&str> = line.split_whitespace().rev().collect();
                while let Some(word) = words.pop() {
                    match word {
                        ":device"   => {
                            input_device_path = Some(String::from(words.pop().expect("[ input ] no input device specified")));
                            continue;
                        },
                        "*"         => keycode += 1,
                        word        => keycode = word.parse().expect("[ input ] failed to parse keycode"),
                    }

                    match words.pop().expect("[ input ] key action specified") {

                        // press on event
                        "press"     => {
                            let parse_code = |source: Option<&str>| {
                                match source {
                                    Some(word)  => {
                                        match word.chars().nth(0).unwrap() {
                                            'b'     => Some(word.chars().nth(1).expect("[ input ] no character specified") as u8),
                                            's'     => Some(32),
                                            ':'     => None,
                                            _       => Some(word.parse().expect("[ input ] failed to parse character")),
                                        }
                                    },
                                    None        => None,
                                }
                            };
                            translation_table[keycode as usize] = Action::Press(parse_code(words.pop()), parse_code(words.pop()), parse_code(words.pop()));
                        },

                        // set modifier on press and release event
                        "set"       => translation_table[keycode as usize] = Action::Set(Modifier::from_slice(words.pop().expect("[ input ] no modifier for set specified"))),

                        // toggle modifier on press event
                        "toggle"    => translation_table[keycode as usize] = Action::Toggle(Modifier::from_slice(words.pop().expect("[ input ] no modifier for toggle specified"))),

                        // undefined action
                        word        => panic!("[ input ] undefined action '{}'", word),
                    }
                }
            }
        }

        // return the new input handler
        InputHandler {
            translation_table:  translation_table,
            input_device:       File::open(input_device_path.expect("[ input ] no input device specified")).expect("[ input ] failed to open input device"),
            modifiers:          Modifier::None as u8,
        }
    }

    // parse keyboard events till a keypress event is returned
    pub fn read_event(&mut self) -> (u8, u8) {
        let mut buffer = [0; 24];
        loop {

            // read all device input
            self.input_device.read_exact(&mut buffer).unwrap();
            if let Some(event) = InputEvent::from_u8(buffer[16]) {
                match event {

                    // keyboard input
                    InputEvent::Keyboard    => {
                        let keycode = buffer[18];
                        let key_state = match KeyState::from_u8(buffer[20]) {
                            Some(state)     => state,
                            None            => continue,
                        };

                        // output raw keycodes
                        #[cfg(feature = "verbose")]
                        println!("[ input ] keyboard event with keycode {} and value {}", keycode, buffer[20]);

                        // translate keycode
                        match self.translation_table[keycode as usize] {

                            // on pressing and on holding a key, send the transpated character
                            Action::Press(character, capital, extra)    => {
                                match key_state {
                                    KeyState::Press | KeyState::Repeat  => {
                                        if self.modifiers & (Modifier::Shift as u8) != 0 {
                                            if self.modifiers & (Modifier::Control as u8) != 0 {
                                                if let Some(extra) = extra {
                                                    return (self.modifiers, extra);
                                                }
                                            }
                                            if let Some(capital) = capital {
                                                return (self.modifiers, capital);
                                            }
                                        }
                                        if let Some(character) = character {
                                            return (self.modifiers, character);
                                        }
                                    },
                                    _                                   => continue,
                                }
                            },

                            // on pressing the key, toggle a modifier
                            Action::Toggle(modifier)                    => {
                                match key_state {
                                    KeyState::Press     => self.modifiers ^= modifier as u8,
                                    _                   => continue,
                                }
                            },

                            // on pressing the key the modifier gets activated, on release it gets deactivated
                            Action::Set(modifier)                       => {
                                match key_state {
                                    KeyState::Release   => self.modifiers &= !(modifier as u8),
                                    KeyState::Press     => self.modifiers |= modifier as u8,
                                    _                   => continue,
                                }
                            },

                            // don't process
                            Action::Drop                                => continue,
                        }
                    }
                }
            }
        }
    }
}
