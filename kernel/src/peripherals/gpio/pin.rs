#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Pull {
    None,
    Down,
    Up,
}

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

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Pin {
    Virtual0,
    Virtual1,
    Virtual2,
    Virtual3,
    Virtual4,
    Virtual5,
    Virtual6,
    Virtual7,
    Virtual8,
    Virtual9,
    Virtual10,
    Virtual11,
    Virtual12,
    Virtual13,
    Virtual14,
    Virtual15,
    Virtual16,
    Virtual17,
    Virtual18,
    Virtual19,
    Virtual20,
    Virtual21,
    Virtual22,
    Virtual23,
    Virtual24,
    Virtual25,
    Virtual26,
    Virtual27,
    Virtual28,
    Virtual29,
    Virtual30,
    Virtual31,
    Virtual32,
    Virtual33,
    Virtual34,
    Virtual35,
    Virtual36,
    Virtual37,
    Virtual38,
    Virtual39,
    Virtual40,
    Virtual41,
    Virtual42,
    Virtual43,
    Virtual44,
    Virtual45,
    Virtual46,
    Virtual47,
    Virtual48,
    Virtual49,
    Virtual50,
    Virtual51,
    Virtual52,
    Virtual53,
}

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
impl Pin {
    pub const Physical3: Pin = Pin::Virtual2;
    pub const Physical5: Pin = Pin::Virtual3;
    pub const Physical7: Pin = Pin::Virtual4;
    pub const Physical8: Pin = Pin::Virtual14;
    pub const Physical10: Pin = Pin::Virtual15;
    pub const Physical11: Pin = Pin::Virtual17;
    pub const Physical12: Pin = Pin::Virtual18;
    pub const Physical13: Pin = Pin::Virtual27;
    pub const Physical15: Pin = Pin::Virtual22;
    pub const Physical16: Pin = Pin::Virtual23;
    pub const Physical18: Pin = Pin::Virtual24;
    pub const Physical19: Pin = Pin::Virtual10;
    pub const Physical21: Pin = Pin::Virtual9;
    pub const Physical22: Pin = Pin::Virtual25;
    pub const Physical23: Pin = Pin::Virtual11;
    pub const Physical24: Pin = Pin::Virtual8;
    pub const Physical26: Pin = Pin::Virtual7;
    pub const Physical29: Pin = Pin::Virtual5;
    pub const Physical31: Pin = Pin::Virtual6;
    pub const Physical32: Pin = Pin::Virtual12;
    pub const Physical33: Pin = Pin::Virtual13;
    pub const Physical35: Pin = Pin::Virtual19;
    pub const Physical36: Pin = Pin::Virtual16;
    pub const Physical37: Pin = Pin::Virtual26;
    pub const Physical38: Pin = Pin::Virtual20;
    pub const Physical40: Pin = Pin::Virtual21;
}
