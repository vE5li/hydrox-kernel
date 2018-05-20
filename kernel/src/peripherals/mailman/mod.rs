// mailbox peripheral addresses
const MAILBOX0_BASE: usize      = super::PERIPHERALS_BASE + 0xb880;
const MAILBOX1_BASE: usize      = super::PERIPHERALS_BASE + 0xb8a0;

// bitmasks
const MAILBOX_EMPTY: u32        = 1 << 30;
const MAILBOX_FULL: u32         = 1 << 31;

// mailbox buffer size in words
const BUFFER_LENGTH: usize      = 32;

// mailbox request buffer
pub type Buffer = [u32; BUFFER_LENGTH];

// mailbox registers layout
#[repr(C)]
#[allow(dead_code)]
struct Registers {
    data:       u32,
    _unused:    [u32; 3],
    poll:       u32,
    sender:     u32,
    status:     u32,
    config:     u32,
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

// mailbox firmware tag
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum MailboxTag {
    SetPowerState,
    GetFramebufferSize,
    SetPhysicalSize,
    SetVirtualSize,
    SetFramebufferDepth,
    AllocateFramebuffer,
    GetFramebufferPitch,
}

// implement mailbox tag
impl MailboxTag {

    // get the code and data layout of a tag
    pub fn layout(&self) -> (u32, u32, u32) {
        match *self {
            MailboxTag::SetPowerState           => (0x38041, 8, 0),
            MailboxTag::GetFramebufferSize      => (0x40003, 8, 0),
            MailboxTag::SetPhysicalSize         => (0x48003, 8, 8),
            MailboxTag::SetVirtualSize          => (0x48004, 8, 8),
            MailboxTag::SetFramebufferDepth     => (0x48005, 4, 4),
            MailboxTag::AllocateFramebuffer     => (0x40001, 8, 4),
            MailboxTag::GetFramebufferPitch     => (0x40008, 4, 0),
        }
    }
}

// wrapper for an array to force alignment
#[repr(align(16))]
pub struct Letter {
    buffer: Buffer,
}

// implement letter
impl Letter {

    // get a cleared letter
    pub fn new() -> Letter {
        Letter {
            buffer: [0; BUFFER_LENGTH]
        }
    }

    // clear the request buffer
    pub fn clear(&mut self) {
        self.buffer[0] = 0;
    }

    // clear the request buffer and set the message size
    pub fn clear_tags(&mut self) {
        self.buffer[0] = 8;
        self.buffer[1] = 0;
    }

    // push a word to the mailbox
    pub fn push(&mut self, data: u32) {
        self.buffer[self.buffer[0] as usize / 4] = data;
        self.buffer[0] += 4;
        assert!(self.buffer[0] as usize <= BUFFER_LENGTH * 4, "mailbox request overflow");
    }

    // push data from a slice as a tag
    pub fn push_tag(&mut self, tag: MailboxTag, data: &[u32]) {
        let (code, request, response) = tag.layout();
        assert!(request / 4 == data.len() as u32, "invalid data for specified tag");
        self.push(code);
        self.push(request);
        self.push(response);

        // push enought data to fit the request and the response
        let mut index: u32 = 0;
        while index < request / 4 {
            self.push(data[index as usize]);
            index += 1;
        }
        while index < response / 4 {
            self.push(0);
            index += 1;
        }
    }

    // push an end tag
    pub fn push_end_tag(&mut self) {
        self.push(0);
    }

    // push request to mailbox
    pub fn send(&self, channel: Channel) {
        unsafe {
            let registers = MAILBOX1_BASE as *mut Registers;
            while (*registers).status & MAILBOX_FULL != 0 {}
            (*registers).data = &self.buffer as *const _ as u32 | channel as u32;
        }
    }

    // pop the requested mailbox response
    pub fn receive(&self, channel: Channel) {
        let mut address: u32 = !(channel as u32);
        unsafe {
            let registers = MAILBOX0_BASE as *mut Registers;
            while address & 0b1111 != channel as u32 {
                while (*registers).status & MAILBOX_EMPTY != 0 {}
                address = (*registers).data;
            };
        }
        assert!(address & !0b1111 == &self.buffer as *const _ as u32, "invalid mailbox response buffer");
    }

    // get the index of a tag
    pub fn tag_index(&self, tag: MailboxTag) -> Option<usize> {
        let (code, _, _) = tag.layout();
        let length = self.buffer.len();
        let mut index = 2;

        // iterate over all the tag headers
        while index < length {
            if self.buffer[index] == code {
                return Some(index);
            }
            index += 3 + (self.buffer[index + 1] >> 2) as usize;
        }
        None
    }

    // get a mutable reference to the request buffer
    pub fn buffer(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
}
