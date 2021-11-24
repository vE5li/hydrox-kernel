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
    GetFirmwareVersion,
    GetBoardModel,
    GetBoardRevision,
    GetBoardSerial,
    GetARMMemory,
    GetVideoCoreMemory,
    GetTemperature,
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
            //                                     tag, request_len, response_len
            MailboxTag::GetFirmwareVersion      => (0x00001, 0, 4),
            MailboxTag::GetBoardModel           => (0x10001, 0, 4),
            MailboxTag::GetBoardRevision        => (0x10002, 0, 4),
            MailboxTag::GetBoardSerial          => (0x10004, 0, 8),
            MailboxTag::GetARMMemory            => (0x10005, 0, 8),
            MailboxTag::GetVideoCoreMemory      => (0x10006, 0, 8),
            MailboxTag::GetTemperature          => (0x30006, 4, 8),
            MailboxTag::SetLEDStatus            => (0x38041, 8, 8),
            MailboxTag::GetFramebufferSize      => (0x40003, 0, 8),
            MailboxTag::SetPhysicalSize         => (0x48003, 8, 8),
            MailboxTag::SetVirtualSize          => (0x48004, 8, 8),
            MailboxTag::SetFramebufferDepth     => (0x48005, 4, 4),
            MailboxTag::SetPixelOrder           => (0x48006, 4, 4),
            MailboxTag::SetAlphaMode            => (0x48007, 4, 4),
            MailboxTag::AllocateFramebuffer     => (0x40001, 4, 8),
            MailboxTag::GetFramebufferPitch     => (0x40008, 0, 4),
            MailboxTag::SetVirtualOffset        => (0x48009, 8, 8),
        }
    }
}

pub enum OnBoardLed {
    Status = 42,
    Power = 130,
}

impl OnBoardLed {

    pub fn status_to_hardware_state(&self, status: bool) -> u32 {
        match self {
            OnBoardLed::Status => return status as u32,
            OnBoardLed::Power => return (!status) as u32, // actual state of the power led is inverted
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum AlphaMode {
    Opaque,
    Transparent,
    Ignored,
}

#[derive(Copy, Clone)]
pub struct FramebufferSize {
    pub width: u32,
    pub height: u32,
}

impl core::fmt::Display for FramebufferSize {

    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        return write!(formatter, "{} x {}", self.width, self.height);
    }
}

#[derive(Copy, Clone)]
pub struct MemoryLayout {
    pub address: u32,
    pub size: u32,
}

impl core::fmt::Display for MemoryLayout {

    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        return write!(formatter, "0x{:x} - 0x{:x} (0x{:x})", self.address, self.address + self.size, self.size);
    }
}

#[repr(align(16))]
pub struct Message<const N: usize> {
    buffer: Buffer<N>,
}

impl<const N: usize> Message<N> {

    pub fn new() -> Self {

        let mut buffer = [0; N];
        buffer[0] = 8;

        return Self { buffer };
    }

    fn push(&mut self, data: u32) {
        self.buffer[self.buffer[0] as usize / 4] = data;
        self.buffer[0] += 4;
        assert!(self.buffer[0] as usize <= N * 4, "mailbox request overflow");
    }

    fn push_tag(&mut self, tag: MailboxTag, data: &[u32]) {
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

    fn send(&self, channel: Channel) {

        let registers = MAILBOX1_BASE as *mut MailboxRegisters;
        let buffer_with_channel = &self.buffer as *const _ as u32 | channel as u32;

        while read_register!(registers, status) & MAILBOX_FULL != 0 {}
        write_register!(registers, data, buffer_with_channel);
    }

    fn receive(&self, channel: Channel) {

        let registers = MAILBOX0_BASE as *mut MailboxRegisters;
        let mut address: u32 = !(channel as u32);

        while address & 0b1111 != channel as u32 {
            while read_register!(registers, status) & MAILBOX_EMPTY != 0 {}
            address = read_register!(registers, data);
        }

        assert!(&self.buffer as *const _ as u32 == address & !0b1111, "invalid mailbox response buffer");
        assert!(self.buffer[1] == 0x80000000, "error processing mailbox message");
    }

    pub fn finalize_send_receive(&mut self, channel: Channel) {
        self.push(0); // push end tag
        self.send(channel);
        self.receive(channel);
    }

    fn tag_index(&self, tag: MailboxTag) -> Option<usize> {
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

    fn buffer_as_struct<T>(&self, tag: MailboxTag) -> &T {
        let index = self.tag_index(tag).expect("failed to find tag"); // add :? for tag ?
        return unsafe { &*(&self.buffer[index + 3] as *const u32 as *const T) };
    }

    pub fn get_firmware_version_request(&mut self) {
        return self.push_tag(MailboxTag::GetFirmwareVersion, &[]);
    }

    pub fn get_firmware_version_response(&self) -> u32 {
        return *self.buffer_as_struct(MailboxTag::GetFirmwareVersion);
    }

    pub fn get_board_model_request(&mut self) {
        return self.push_tag(MailboxTag::GetBoardModel, &[]);
    }

    pub fn get_board_model_response(&self) -> u32 {
        return *self.buffer_as_struct(MailboxTag::GetBoardModel);
    }

    pub fn get_board_revision_request(&mut self) {
        return self.push_tag(MailboxTag::GetBoardRevision, &[]);
    }

    pub fn get_board_revision_response(&self) -> u32 {
        return *self.buffer_as_struct(MailboxTag::GetBoardRevision);
    }

    pub fn get_board_serial_request(&mut self) {
        return self.push_tag(MailboxTag::GetBoardSerial, &[]);
    }

    pub fn get_board_serial_response(&self) -> u64 {
        let board_serial = self.buffer_as_struct::<[u32; 2]>(MailboxTag::GetBoardSerial);
        let upper = (board_serial[1] as u64) << 32;
        let lower = board_serial[0] as u64;
        return upper | lower;
    }

    pub fn get_arm_memory_request(&mut self) {
        return self.push_tag(MailboxTag::GetARMMemory, &[]);
    }

    pub fn get_arm_memory_response(&self) -> MemoryLayout {
        return *self.buffer_as_struct(MailboxTag::GetARMMemory);
    }

    pub fn get_video_core_memory_request(&mut self) {
        return self.push_tag(MailboxTag::GetVideoCoreMemory, &[]);
    }

    pub fn get_video_core_memory_response(&self) -> MemoryLayout {
        return *self.buffer_as_struct(MailboxTag::GetVideoCoreMemory);
    }

    pub fn set_led_status_request(&mut self, led: OnBoardLed, status: bool) {
        let hardware_state = led.status_to_hardware_state(status);
        return self.push_tag(MailboxTag::SetLEDStatus, &[led as u32, hardware_state]);
    }

    pub fn get_framebuffer_size_request(&mut self) {
        return self.push_tag(MailboxTag::GetFramebufferSize, &[]);
    }

    pub fn get_framebuffer_size_response(&self) -> FramebufferSize {
        return *self.buffer_as_struct(MailboxTag::GetFramebufferSize);
    }

    pub fn set_physical_size_request(&mut self, framebuffer_size: FramebufferSize) {
        return self.push_tag(MailboxTag::SetPhysicalSize, &[framebuffer_size.width, framebuffer_size.height]);
    }

    pub fn set_physical_size_response(&mut self) -> FramebufferSize {
        return *self.buffer_as_struct(MailboxTag::SetPhysicalSize);
    }

    pub fn set_virtual_size_request(&mut self, framebuffer_size: FramebufferSize) {
        return self.push_tag(MailboxTag::SetVirtualSize, &[framebuffer_size.width, framebuffer_size.height]);
    }

    pub fn set_virtual_size_response(&mut self) -> FramebufferSize {
        return *self.buffer_as_struct(MailboxTag::SetVirtualSize);
    }

    pub fn set_virtual_offset_request(&mut self, offset_x: u32, offset_y: u32) {
        return self.push_tag(MailboxTag::SetVirtualOffset, &[offset_x, offset_y]);
    }

    pub fn set_framebuffer_depth_request(&mut self, depth: u32) {
        return self.push_tag(MailboxTag::SetFramebufferDepth, &[depth]);
    }

    pub fn set_framebuffer_depth_response(&self) -> u32 {
        return *self.buffer_as_struct(MailboxTag::SetFramebufferDepth);
    }

    pub fn set_alpha_mode_request(&mut self, alpha_mode: AlphaMode) {
        return self.push_tag(MailboxTag::SetAlphaMode, &[alpha_mode as u32]);
    }

    pub fn set_alpha_mode_response(&self) -> AlphaMode {
        return *self.buffer_as_struct(MailboxTag::SetAlphaMode);
    }

    pub fn set_pixel_order_request(&mut self, pixel_order: u32) {
        return self.push_tag(MailboxTag::SetPixelOrder, &[pixel_order]);
    }

    pub fn set_pixel_order_response(&self) -> u32 {
        return *self.buffer_as_struct(MailboxTag::SetPixelOrder);
    }

    pub fn allocate_framebuffer_request(&mut self, alignment: u32) {
        return self.push_tag(MailboxTag::AllocateFramebuffer, &[alignment]);
    }

    pub fn allocate_framebuffer_response(&self) -> MemoryLayout {
        return *self.buffer_as_struct(MailboxTag::AllocateFramebuffer);
    }

    pub fn get_framebuffer_pitch_request(&mut self) {
        return self.push_tag(MailboxTag::GetFramebufferPitch, &[]);
    }

    pub fn get_framebuffer_pitch_response(&self) -> u32 {
        return *self.buffer_as_struct(MailboxTag::GetFramebufferPitch);
    }

    pub fn get_temperature_request(&mut self) {
        return self.push_tag(MailboxTag::GetTemperature, &[1]);
    }

    pub fn get_temperature_response(&self) -> u32 {
        let buffer = self.buffer_as_struct::<[u32; 2]>(MailboxTag::GetTemperature);
        return buffer[1];
    }
}
