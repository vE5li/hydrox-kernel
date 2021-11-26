mod pin;

pub use self::pin::{ Pin, Pull, Function };

const GPIO_BASE: usize = super::PERIPHERALS_BASE + 0x200000;

#[repr(C)]
#[allow(dead_code)]
struct GPIORegisters {
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

pub fn idle(cycles: usize) {
    for _ in 0..cycles {
        unsafe { asm!("nop"); }
    }
}

pub fn set_function(pin: Pin, function: Function) {

    let registers = GPIO_BASE as *mut GPIORegisters;
    let left_shift = (pin as usize % 10) * 3;
    let index = pin as usize / 10;

    let mut value = read_register!(registers, fsel, index);
    value &= !(0b111 << left_shift);
    value |= (function as u32) << left_shift;
    write_register!(registers, fsel, index, value);
}

pub fn set_state(pin: Pin, state: bool) {

    let registers = GPIO_BASE as *mut GPIORegisters;
    let index = (pin as usize > 32) as usize; // convert bool to 0 or 1
    let left_shift = pin as usize - (32 * index);

    match state {
        true => write_register!(registers, set, index, 1 << left_shift),
        false => write_register!(registers, clr, index, 1 << left_shift),
    }
}

pub fn set_pull(pin: Pin, pull: Pull) {

    let registers = GPIO_BASE as *mut GPIORegisters;
    let index = (pin as usize > 32) as usize; // convert bool to 0 or 1
    let left_shift = pin as usize - (32 * index);

    write_register!(registers, pud, pull as u32);
    idle(150);
    write_register!(registers, pudclk, index, 1 << left_shift);
    idle(150);
    write_register!(registers, pud, 0);
    write_register!(registers, pudclk, index, 0);
}
