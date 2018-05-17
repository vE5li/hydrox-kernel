// bitmasks
const MAILBOX_EMPTY: u32        = 1 << 30;
const MAILBOX_FULL: u32         = 1 << 31;

// mailbox request buffer
pub type Buffer = [u32; 32];

// static instance
static mut LETTER: Letter = Letter { buffer: [0; 32] };

// wrapper for an array to force alignment
#[repr(align(16))]
struct Letter {
    pub buffer: Buffer,
}

// mailbox channels
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Channel {
    Power,
    FrameBuffer,
    VUART,
    VCHIQ,
    LED,
    Button,
    Touchscreen,
    Count,
    Tags,
    GPU,
}

// mailbox registers layout
#[allow(dead_code)]
#[repr(C)]
struct Mailbox {
    data:       u32,
    _unused:    [u32; 3],
    poll:       u32,
    sender:     u32,
    status:     u32,
    config:     u32,
}

// get a mutable reference to the request buffer
pub fn buffer() -> &'static mut Buffer {
    unsafe { &mut LETTER.buffer }
}

// push request to mailbox
pub fn push(channel: Channel, data: u32) {
    assert!(data & 0b1111 == 0, "attempted to push invalid data to the mailbox");
    unsafe {
        let mailbox = peripheral!(mailbox1) as *mut Mailbox;
        while (*mailbox).status & MAILBOX_FULL != 0 {}
        (*mailbox).data = data | channel as u32;
    }
}

// pop the requested mailbox response
pub fn pop(channel: Channel) -> u32 {
    let mut data: u32 = !(channel as u32);
    unsafe {
        let mailbox = peripheral!(mailbox0) as *mut Mailbox;
        while data & 0b1111 != channel as u32 {
            while (*mailbox).status & MAILBOX_EMPTY != 0 {}
            data = (*mailbox).data;
        };
    }
    data & !0b1111
}

// push data from a slice as a tag
pub fn push_tag(data: &[u32]) {
    let buffer = unsafe { &mut LETTER.buffer };
    buffer[0] = (data.len() as u32 + 3) * 4;
    buffer[1] = 0;
    for (index, word) in data.iter().enumerate() {
        buffer[index + 2] = *word;
    }
    buffer[data.len() + 2] = 0;
    push(Channel::Tags, buffer as *const _ as u32);
}

// pop a reponse from the tag channel
pub fn pop_tag() -> &'static Buffer {
    pop(Channel::Tags);
    unsafe { &LETTER.buffer }
}
