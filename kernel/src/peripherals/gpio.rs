// gpio resistor state
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Pull {
    Off,
    Down,
    Up,
}

// gpio pin functions
#[allow(dead_code)]
#[derive(Copy, Clone)]
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

// all gpio pins
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Pin {
    V0      = 0,
    V1      = 1,
    V2      = 2,
    V3      = 3,
    V4      = 4,
    V5      = 5,
    V6      = 6,
    V7      = 7,
    V8      = 8,
    V9      = 9,
    V10     = 10,
    V11     = 11,
    V12     = 12,
    V13     = 13,
    V14     = 14,
    V15     = 15,
    V16     = 16,
    V17     = 17,
    V18     = 18,
    V19     = 19,
    V20     = 20,
    V21     = 21,
    V22     = 22,
    V23     = 23,
    V24     = 24,
    V25     = 25,
    V26     = 26,
    V27     = 27,
    V28     = 28,
    V29     = 29,
    V30     = 30,
    V31     = 31,
    V32     = 32,
    V33     = 33,
    V34     = 34,
    V35     = 35,
    V36     = 36,
    V37     = 37,
    V38     = 38,
    V39     = 39,
    V40     = 40,
    V41     = 41,
    V42     = 42,
    V43     = 43,
    V44     = 44,
    V45     = 45,
    V46     = 46,
    V47     = 47,
    V48     = 48,
    V49     = 49,
    V50     = 50,
    V51     = 51,
    V52     = 52,
    V53     = 53,
}

// gpio register layout
#[repr(C)]
#[allow(dead_code)]
struct Registers {
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
pub fn set_function(pin: Pin, function: Function) {
    let pin = pin as usize;
    unsafe {
        let offset = (pin % 10) * 3;
        let registers = peripheral!(gpio) as *mut Registers;
        (*registers).fsel[pin / 10] &= !(0b111 << offset);
        (*registers).fsel[pin / 10] |= (function as u32) << offset;
    }
}

// set the state of a gpio pin
pub fn set_state(pin: Pin, state: bool) {
    let pin = pin as usize;
    unsafe {
        let registers = peripheral!(gpio) as *mut Registers;
        if state {
            match pin {
                0...31 => (*registers).set[0] = 1 << pin,
                _ => (*registers).set[1] = 1 << (pin - 32),
            }
        } else {
            match pin {
                0...31 => (*registers).clr[0] = 1 << pin,
                _ => (*registers).clr[1] = 1 << (pin - 32),
            }
        }
    }
}

// set the resistor for a gpio pin
pub fn set_pull(pin: Pin, pull: Pull) {
    let pin = pin as usize;
    unsafe {
        let registers = peripheral!(gpio) as *mut Registers;
        (*registers).pud = pull as u32;
        for _ in 0..150 {}
        if pin < 32 {
            (*registers).pudclk[0] = 1 << pin;
            for _ in 0..150 {}
            (*registers).pudclk[0] = 0;
        } else {
            (*registers).pudclk[1] = 1 << (pin - 32);
            for _ in 0..150 {}
            (*registers).pudclk[1] = 0;
        }
        (*registers).pud = 0;
    }
}
