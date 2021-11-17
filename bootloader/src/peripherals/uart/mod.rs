use core::ptr::{ read_volatile, write_volatile };

use peripherals::gpio::{ Pin, Pull, Function, set_function, set_pull };

const UART_BASE: usize = super::PERIPHERALS_BASE + 0x215000;
const UART_CLOCK: u32 = 500000000;
const BAUD_RATE: u32 = 115200;

#[repr(C)]
#[allow(dead_code)]
struct Registers {
    aux_irq: u32,
    aux_enables: u32, // 8
    _reserved0: [u16; 27], // padding
    aux_mu_io_reg: u32, // 64
    aux_mu_ier_reg: u32,
    aux_mu_iir_reg: u32,
    aux_mu_lcr_reg: u32,
    aux_mu_mcr_reg: u32,
    aux_mu_lsr_reg: u32,
    aux_mu_msr_reg: u32,
    aux_mu_scratch: u32,
    aux_mu_cntl_reg: u32,
    aux_mu_stat_reg: u32,
    aux_mu_baud_reg: u32,
//    aux_spi1: SpiRegisters,
//    aux_spi2: SpiRegisters,
}

pub fn initialize() {

    unsafe {

        let registers = UART_BASE as *mut Registers;

        write_volatile(&mut (*registers).aux_enables, 1);
        write_volatile(&mut (*registers).aux_mu_ier_reg, 0);
        write_volatile(&mut (*registers).aux_mu_cntl_reg, 0);
        write_volatile(&mut (*registers).aux_mu_lcr_reg, 3);
        write_volatile(&mut (*registers).aux_mu_mcr_reg, 0);
        write_volatile(&mut (*registers).aux_mu_ier_reg, 0);
        write_volatile(&mut (*registers).aux_mu_iir_reg, 0xC6);
        write_volatile(&mut (*registers).aux_mu_baud_reg, (UART_CLOCK / (BAUD_RATE * 8)) - 1);

        set_pull(Pin::V14, Pull::None);
        set_function(Pin::V14, Function::Alternate5);
        set_pull(Pin::V15, Pull::None);
        set_function(Pin::V15, Function::Alternate5);

        write_volatile(&mut (*registers).aux_mu_cntl_reg, 3);
    }

    //success!("uart initialized");
}

pub fn is_write_byte_ready() -> bool {
    unsafe {
        let registers = UART_BASE as *mut Registers;
        return read_volatile(&(*registers).aux_mu_lsr_reg) & 0x20 != 0;
    }
}

pub fn write_character_blocking(character: char) {

    while !is_write_byte_ready() {};

    unsafe {
        let registers = UART_BASE as *mut Registers;
        write_volatile(&mut (*registers).aux_mu_io_reg, character as u32);
    }
}
