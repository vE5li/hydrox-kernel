extern {
    // log a non-c string
    pub fn log_character(character: u8);

    // get input from the user
    pub fn read_event() -> u16;

    // reboot the device
    pub fn reboot() -> !;

    // base address of the broadcom preipherals
    pub static interface_peripherals_base: usize;
}
