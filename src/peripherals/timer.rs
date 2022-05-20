const TIMER_BASE: usize = super::PERIPHERALS_BASE + 0x3000;

const CLOCK_HZ: u32 = 1000000;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Timer {
    Timer1,
    Timer2,
    Timer3,
    Timer4,
}

#[repr(C)]
#[allow(dead_code)]
struct TimerRegisters {
    control_status: u32,
    counter_lower: u32,
    counter_heigher: u32,
    compare: [u32; 4],
}

pub fn set_timer(timer: Timer, seconds: u32) {

    let timer_registers = TIMER_BASE as *mut TimerRegisters;

    let timer_value = read_register!(timer_registers, counter_lower).wrapping_add(CLOCK_HZ * seconds);
    write_register!(timer_registers, compare, timer as usize, timer_value);
}

pub fn clear_timer_interrupt(timer: Timer) {

    let timer_registers = TIMER_BASE as *mut TimerRegisters;

    let control_status = read_register!(timer_registers, control_status);
    write_register!(timer_registers, control_status, control_status | 1 << timer as u32);
}
