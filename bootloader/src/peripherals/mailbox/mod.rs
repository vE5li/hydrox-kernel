use core::cmp::max;

const MAILBOX0_BASE: usize = super::PERIPHERALS_BASE + 0xb880;
const MAILBOX1_BASE: usize = super::PERIPHERALS_BASE + 0xb8a0;

const MAILBOX_EMPTY: u32 = 1 << 30;
const MAILBOX_FULL: u32 = 1 << 31;

pub type Buffer<const N: usize> = [u32; N];

#[repr(C)]
#[allow(dead_code)]
struct MailboxRegisters {
    data:       u32,
    _unused:    [u32; 3],
    poll:       u32,
    sender:     u32,
    status:     u32,
    config:     u32,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Channel {
    PowerManagment,
    Framebuffer,
    VirtualUART,
    VCHIQ, // kernel to video core communication
    LEDs,
    Buttons,
    TouchScreen,
    Count, // unknown
    Tags, // property tags
    GPU, // property tags
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum MailboxTag {
    SetLEDStatus,
    GetFramebufferSize,
    SetPhysicalSize,
    SetVirtualSize,
    SetFramebufferDepth,
    SetPixelOrder,
    SetAlphaMode,
    AllocateFramebuffer,
    GetFramebufferPitch,
    SetVirtualOffset,
}

impl MailboxTag {

    pub fn layout(&self) -> (u32, u32, u32) {
        match *self {
            //MailboxTag::SetPowerState           => (0x28001, 8, 0),
            MailboxTag::SetLEDStatus            => (0x38041, 8, 8),
            MailboxTag::GetFramebufferSize      => (0x40003, 0, 8),
            MailboxTag::SetPhysicalSize         => (0x48003, 8, 8),
            MailboxTag::SetVirtualSize          => (0x48004, 8, 8),
            MailboxTag::SetFramebufferDepth     => (0x48005, 4, 4),
            MailboxTag::SetPixelOrder           => (0x48006, 4, 4),
            MailboxTag::SetAlphaMode            => (0x48007, 4, 4),
            MailboxTag::AllocateFramebuffer     => (0x40001, 4, 8), // 4, 8
            MailboxTag::GetFramebufferPitch     => (0x40008, 0, 4), // 0, 4
            MailboxTag::SetVirtualOffset        => (0x48009, 8, 8),
        }
    }
}

#[repr(align(16))]
pub struct Message<const N: usize> {
    buffer: Buffer<N>,
}

impl<const N: usize> Message<N> {

    pub fn new() -> Self {
        Self {
            buffer: [0; N]
        }
    }

    pub fn clear_tags(&mut self) {
        self.buffer[0] = 8;
        self.buffer[1] = 0;
    }

    pub fn push(&mut self, data: u32) {
        self.buffer[self.buffer[0] as usize / 4] = data;
        self.buffer[0] += 4;
        assert!(self.buffer[0] as usize <= N * 4, "mailbox request overflow");
    }

    pub fn push_tag(&mut self, tag: MailboxTag, data: &[u32]) {
        let (code, request, response) = tag.layout();
        assert!(request / 4 == data.len() as u32, "invalid data for specified tag");
        self.push(code);
        self.push(max(request, response)); // buffer size
        self.push(request); // request size

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

    pub fn push_end_tag(&mut self) {
        self.push(0);
    }

    pub fn send(&self, channel: Channel) {

        let registers = MAILBOX1_BASE as *mut MailboxRegisters;
        let buffer_with_channel = &self.buffer as *const _ as u32 | channel as u32;

        while read_register!(registers, status) & MAILBOX_FULL != 0 {}
        write_register!(registers, data, buffer_with_channel);
    }

    pub fn receive(&self, channel: Channel) {

        let registers = MAILBOX0_BASE as *mut MailboxRegisters;
        let mut address: u32 = !(channel as u32);

        while address & 0b1111 != channel as u32 {
            while read_register!(registers, status) & MAILBOX_EMPTY != 0 {}
            address = read_register!(registers, data);
        }

        if self.buffer[1] == 0x80000001 {
            error!("failed to parse message"); // temporary while panic doesnt output the message
        }

        assert!(&self.buffer as *const _ as u32 == address & !0b1111, "invalid mailbox response buffer");
        assert!(self.buffer[1] == 0x80000000, "error processing the message");
    }

    pub fn tag_index(&self, tag: MailboxTag) -> Option<usize> {
        let (code, _, _) = tag.layout();
        let mut index = 2;

        while index < N {

            if self.buffer[index] == 0x0 { // end tag
                break;
            }

            if self.buffer[index] == code {
                return Some(index);
            }

            let buffer_size = self.buffer[index + 1] / 4;
            index += 3 + buffer_size as usize;
        }

        return None;
    }

    pub fn buffer(&mut self) -> &mut Buffer<N> {
        &mut self.buffer
    }
}
