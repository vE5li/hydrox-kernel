extern {

    // log a non-c string
    pub fn log_character(character: u8);

    // get input from the user
    pub fn read_event() -> u16;
}
