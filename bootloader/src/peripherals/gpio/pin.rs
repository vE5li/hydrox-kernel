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
    V0 = 0,
    V1 = 1,
    V2 = 2,
    V3 = 3,
    V4 = 4,
    V5 = 5,
    V6 = 6,
    V7 = 7,
    V8 = 8,
    V9 = 9,
    V10 = 10,
    V11 = 11,
    V12 = 12,
    V13 = 13,
    V14 = 14,
    V15 = 15,
    V16 = 16,
    V17 = 17,
    V18 = 18,
    V19 = 19,
    V20 = 20,
    V21 = 21,
    V22 = 22,
    V23 = 23,
    V24 = 24,
    V25 = 25,
    V26 = 26,
    V27 = 27,
    V28 = 28,
    V29 = 29,
    V30 = 30,
    V31 = 31,
    V32 = 32,
    V33 = 33,
    V34 = 34,
    V35 = 35,
    V36 = 36,
    V37 = 37,
    V38 = 38,
    V39 = 39,
    V40 = 40,
    V41 = 41,
    V42 = 42,
    V43 = 43,
    V44 = 44,
    V45 = 45,
    V46 = 46,
    V47 = 47,
    V48 = 48,
    V49 = 49,
    V50 = 50,
    V51 = 51,
    V52 = 52,
    V53 = 53,
}

#[allow(dead_code)]
impl Pin {
    pub const P3: Pin = Pin::V2;
    pub const P5: Pin = Pin::V3;
    pub const P7: Pin = Pin::V4;
    pub const P8: Pin = Pin::V14;
    pub const P10: Pin  = Pin::V15;
    pub const P11: Pin  = Pin::V17;
    pub const P12: Pin  = Pin::V18;
    pub const P13: Pin  = Pin::V27;
    pub const P15: Pin  = Pin::V22;
    pub const P16: Pin  = Pin::V23;
    pub const P18: Pin  = Pin::V24;
    pub const P19: Pin  = Pin::V10;
    pub const P21: Pin  = Pin::V9;
    pub const P22: Pin  = Pin::V25;
    pub const P23: Pin  = Pin::V11;
    pub const P24: Pin  = Pin::V8;
    pub const P26: Pin  = Pin::V7;
    pub const P29: Pin  = Pin::V5;
    pub const P31: Pin  = Pin::V6;
    pub const P32: Pin  = Pin::V12;
    pub const P33: Pin  = Pin::V13;
    pub const P35: Pin  = Pin::V19;
    pub const P36: Pin  = Pin::V16;
    pub const P37: Pin  = Pin::V26;
    pub const P38: Pin  = Pin::V20;
    pub const P40: Pin  = Pin::V21;
}
