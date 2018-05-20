mod pin;

pub use self::pin::{ Pin, Pull, Function };

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

// idle for at least 150 cycles
pub fn idle() {
    for _ in 0..150 {}
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
                0...31  => (*registers).set[0] = 1 << pin,
                _       => (*registers).set[1] = 1 << (pin - 32),
            }
        } else {
            match pin {
                0...31  => (*registers).clr[0] = 1 << pin,
                _       => (*registers).clr[1] = 1 << (pin - 32),
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

        // set the pull
        idle();
        match pin {
            0..32   => (*registers).pudclk[0] = 1 << pin,
            _       => (*registers).pudclk[1] = 1 << (pin - 32),
        }
        idle();

        // clear previously used registers
        (*registers).pud = 0;
        match pin {
            0..32   => (*registers).pudclk[0] = 0,
            _       => (*registers).pudclk[1] = 0,
        }
    }
}
