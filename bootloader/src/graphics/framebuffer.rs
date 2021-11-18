use core::ptr::write_unaligned;

use peripherals::mailbox::{ Message, Channel, MailboxTag };

pub struct Framebuffer {
    pub address: usize,
    pub pitch: usize,
    pub width: usize,
    pub height: usize,
}

impl Framebuffer {

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        let offset = (y * (self.width * 4)) + (x * 4);
        unsafe { *((self.address + offset) as *const u32 as *mut u32) = color; }
    }

    pub fn draw_rectangle(&mut self, x_position: usize, y_position: usize, width: usize, height: usize, border_color: u32, fill_color: u32) {
        for y in y_position..y_position + height + 1 {
            for x in x_position..x_position + width + 1 {
                if (x == x_position || x == x_position + width + 1) || (y == y_position || y == y_position + height + 1) {
                    self.draw_pixel(x, y, border_color);
                } else {
                    self.draw_pixel(x, y, fill_color);
                }
            }
        }
    }
}

pub fn initialize() -> Framebuffer {

    let mut message = Message::<32>::new();
    message.clear_tags();
    message.push_tag(MailboxTag::GetFramebufferSize, &[]);
    message.push_end_tag();
    message.send(Channel::Tags);
    message.receive(Channel::Tags);

    let mut width = message.buffer()[5];
    let mut height = message.buffer()[6];

    log!("[ graphics ] monitor width: ");
    ::peripherals::logger::log_hex(width as usize);
    log!("\n\r");

    log!("[ graphics ] monitor height: ");
    ::peripherals::logger::log_hex(height as usize);
    log!("\n\r");

    //if width | height == 0 { // OR: early return without creating a frame buffer
    //    width = 640;
    //    height = 480;
    //}

    let mut message = Message::<64>::new();
    message.clear_tags();
    message.push_tag(MailboxTag::SetPhysicalSize, &[width, height]);
    message.push_tag(MailboxTag::SetVirtualSize, &[width, height]);
    message.push_tag(MailboxTag::SetVirtualOffset, &[0, 0]);
    message.push_tag(MailboxTag::SetFramebufferDepth, &[16]);
    message.push_tag(MailboxTag::SetAlphaMode, &[2]); // ignore alpha channel
    message.push_tag(MailboxTag::SetPixelOrder, &[1]);
    message.push_tag(MailboxTag::AllocateFramebuffer, &[4096]);
    message.push_tag(MailboxTag::GetFramebufferPitch, &[]);
    message.push_end_tag();
    message.send(Channel::Tags);
    message.receive(Channel::Tags);

    let index = message.tag_index(MailboxTag::SetFramebufferDepth).expect("invalid frambuffer depth response");
    let depth = message.buffer()[index + 3] as usize;

    let index = message.tag_index(MailboxTag::SetAlphaMode).expect("invalid frambuffer alpha mode response");
    let alpha_mode = message.buffer()[index + 3] as usize;

    let index = message.tag_index(MailboxTag::SetPixelOrder).expect("invalid frambuffer pixel order response");
    let pixel_order = message.buffer()[index + 3] as usize;

    let index = message.tag_index(MailboxTag::AllocateFramebuffer).expect("invalid allocate frambuffer response");
    let address = message.buffer()[index + 3] as usize & 0x3FFFFFFF;
    let size = message.buffer()[index + 4] as usize;

    let index = message.tag_index(MailboxTag::GetFramebufferPitch).expect("invalid get frambuffer pitch response");
    let pitch = message.buffer()[index + 3] as usize;

    log!("[ graphics ] framebuffer depth: ");
    ::peripherals::logger::log_hex(depth);
    log!("\n\r");
    log!("[ graphics ] framebuffer alpha mode: ");
    ::peripherals::logger::log_hex(alpha_mode);
    log!("\n\r");
    log!("[ graphics ] framebuffer pixel order: ");
    ::peripherals::logger::log_hex(pixel_order);
    log!("\n\r");
    log!("[ graphics ] framebuffer address: ");
    ::peripherals::logger::log_hex(address);
    log!("\n\r");
    log!("[ graphics ] framebuffer size: ");
    ::peripherals::logger::log_hex(size);
    log!("\n\r");
    log!("[ graphics ] framebuffer pitch: ");
    ::peripherals::logger::log_hex(pitch);
    log!("\n\r");

    success!("framebuffer initialized");

    return Framebuffer { address, pitch, width: width as usize, height: height as usize };
}
