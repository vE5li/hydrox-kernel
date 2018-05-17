// gpio resistor state
#[allow(dead_code)]
pub enum Pull {
    Off,
    Down,
    Up,
}

// gpio pin functions
#[allow(dead_code)]
pub enum Function {
    Input,
    Output,
    Alternate5,
    Alternate4,
    Alternate0,
    Alternate1,
    Alternate2,
    Alternate3,
}

// gpio register layout
#[allow(dead_code)]
#[repr(C)]
struct GPIO {
    fsel:           [u32; 6],
    _reserved0:     u32,
    set:            [u32; 2],
    _reserved1:     u32,
    clr:            [u32; 2],
    _reserved2:     u32,
    lev:            [u32; 2],
    _reserved3:     u32,
    eds:            [u32; 2],
    _reserved4:     u32,
    ren:            [u32; 2],
    _reserved5:     u32,
    fen:            [u32; 2],
    _reserved6:     u32,
    hen:            [u32; 2],
    _reserved7:     u32,
    len:            [u32; 2],
    _reserved8:     u32,
    aren:           [u32; 2],
    _reserved9:     u32,
    afen:           [u32; 2],
    _reserved10:    u32,
    pud:            u32,
    pudclk:         [u32; 2],
}

// select a gpio pin function
pub fn set_function(pin: usize, function: Function) {
    unsafe {
        let offset = (pin % 10) * 3;
        let interface = peripheral!(gpio) as *mut GPIO;
        (*interface).fsel[pin / 10] &= !(0b111 << offset);
        (*interface).fsel[pin / 10] |= (function as u32) << offset;
    }
}

// set the state of a gpio pin
pub fn set_state(pin: usize, state: bool) {
    unsafe {
        let interface = peripheral!(gpio) as *mut GPIO;
        if state {
            match pin {
                0...31  => (*interface).set[0] = 1 << pin,
                _       => (*interface).set[1] = 1 << (pin - 32),
            }
        } else {
            match pin {
                0...31  => (*interface).clr[0] = 1 << pin,
                _       => (*interface).clr[1] = 1 << (pin - 32),
            }
        }
    }
}

// set the resistor for a gpio pin
pub fn set_pull(pin: usize, pull: Pull) {
    unsafe {
        let interface = peripheral!(gpio) as *mut GPIO;
        (*interface).pud = pull as u32;
        for _ in 0..150 {}
        if pin < 32 {
            (*interface).pudclk[0] = 1 << pin;
            for _ in 0..150 {}
            (*interface).pudclk[0] = 0;
        } else {
            (*interface).pudclk[1] = 1 << (pin - 32);
            for _ in 0..150 {}
            (*interface).pudclk[1] = 0;
        }
        (*interface).pud = 0;
    }
}
