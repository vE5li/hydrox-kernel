use peripherals::gpio::*;

const UART_BASE: usize = super::PERIPHERALS_BASE + 0x215000;
const UART_CLOCK: u32 = 500000000;
const BAUD_RATE: u32 = 115200;

#[repr(C)]
#[allow(dead_code)]
struct UARTRegisters {
    aux_irq: u32,
    aux_enables: u32,
    _reserved: [u32; 14],
    aux_mu_io_reg: u32,
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

    let registers = UART_BASE as *mut UARTRegisters;

    write_register!(registers, aux_enables, 0x1);
    write_register!(registers, aux_mu_ier_reg, 0);
    write_register!(registers, aux_mu_cntl_reg, 0);
    write_register!(registers, aux_mu_lcr_reg, 0x3);
    write_register!(registers, aux_mu_mcr_reg, 0);
    write_register!(registers, aux_mu_ier_reg, 0);
    write_register!(registers, aux_mu_iir_reg, 0xC6);
    write_register!(registers, aux_mu_baud_reg, (UART_CLOCK / (BAUD_RATE * 8)) - 1);

    set_pull(Pin::V14, Pull::None);
    set_function(Pin::V14, Function::Alternate5);
    set_pull(Pin::V15, Pull::None);
    set_function(Pin::V15, Function::Alternate5);

    write_register!(registers, aux_mu_cntl_reg, 0x3);

    success!("uart initialized");
}

pub fn write_character_blocking(character: char) {

    let registers = UART_BASE as *mut UARTRegisters;

    if character == '\n' {
        write_character_blocking('\r');
    }

    while read_register!(registers, aux_mu_lsr_reg) & 0x20 == 0 { };
    write_register!(registers, aux_mu_io_reg, character as u32);
}

pub fn read_character_blocking() -> char {
    let registers = UART_BASE as *mut UARTRegisters;
    while read_register!(registers, aux_mu_lsr_reg) & 0x01 == 0 { };
    return read_register!(registers, aux_mu_io_reg) as u8 as char;
}
