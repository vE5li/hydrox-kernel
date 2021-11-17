mod pin;

use core::ptr::{ read_volatile, write_volatile };

pub use self::pin::{ Pin, Pull, Function };

const GPIO_BASE: usize = super::PERIPHERALS_BASE + 0x200000;

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

pub fn idle(cycles: usize) {
    for _ in 0..cycles {
        unsafe {
            asm!("nop");
        }
    }
}

pub fn set_function(pin: Pin, function: Function) {
    let pin = pin as usize;
    unsafe {
        let registers = GPIO_BASE as *mut Registers;
        let offset = (pin % 10) * 3;
        let fsel = &mut (*registers).fsel[pin / 10];

        let mut value = read_volatile(fsel);
        value &= !(0b111 << offset);
        value |= (function as u32) << offset;
        write_volatile(fsel, value);
    }
}

pub fn set_state(pin: Pin, state: bool) {
    let pin = pin as usize;
    unsafe {
        let registers = GPIO_BASE as *mut Registers;
        if state {
            match pin {
                0..=31  => write_volatile(&mut (*registers).set[0], 1 << pin),
                _       => write_volatile(&mut (*registers).set[1], 1 << (pin - 32)),
            }
        } else {
            match pin {
                0..=31  => write_volatile(&mut (*registers).clr[0], 1 << pin),
                _       => write_volatile(&mut (*registers).clr[1], 1 << (pin - 32)),
            }
        }
    }
}

pub fn set_pull(pin: Pin, pull: Pull) {
    let pin = pin as usize;
    unsafe {
        let registers = GPIO_BASE as *mut Registers;
        write_volatile(&mut (*registers).pud, pull as u32);

        // set the pull
        idle(150);
        match pin {
            0..=31  => write_volatile(&mut (*registers).pudclk[0], 1 << pin),
            _       => write_volatile(&mut (*registers).pudclk[1], 1 << (pin - 32)),
        }
        idle(150);

        // clear previously used registers
        write_volatile(&mut (*registers).pud, 0);
        match pin {
            0..=31  => write_volatile(&mut (*registers).pudclk[0], 0),
            _       => write_volatile(&mut (*registers).pudclk[1], 0),
        }
    }
}
