use crate::peripherals::timer::{ Timer, set_timer, clear_timer_interrupt };

const INTERRUPT_BASE: usize = super::PERIPHERALS_BASE + 0xB200;

const SYSTEM_TIMER_INTERRUPT: [u32; 4] = [1, 2, 4, 8];

#[repr(C)]
#[allow(dead_code)]
struct InterruptRegisters {
    pending: [u32; 3],
    _reserved0: u32,
    enable: [u32; 3],
    _reserved1: u32,
    disable: [u32; 3],
}

pub fn enable_interrupt_controller() {
    let interrupt_registers = INTERRUPT_BASE as *mut InterruptRegisters;
    write_register!(interrupt_registers, enable, 0, SYSTEM_TIMER_INTERRUPT[1] | SYSTEM_TIMER_INTERRUPT[3]);
}

//pub fn disable_interrupt_controller() {
//    let interrupt_registers = INTERRUPT_BASE as *mut InterruptRegisters;
//    write_register!(interrupt_registers, enable, 0, 0);
//}

#[no_mangle]
pub extern fn handle_irq() {

    let interrupt_registers = INTERRUPT_BASE as *mut InterruptRegisters;
    let pending = read_register!(interrupt_registers, pending, 0);

    if pending & SYSTEM_TIMER_INTERRUPT[1] != 0 {

        success!("timer 1 interrupt!");

        clear_timer_interrupt(Timer::Timer1);
        set_timer(Timer::Timer1, 2);
    }

    if pending & SYSTEM_TIMER_INTERRUPT[3] != 0 {

        success!("timer 3 interrupt!");

        clear_timer_interrupt(Timer::Timer3);
        set_timer(Timer::Timer3, 4);
    }
}
