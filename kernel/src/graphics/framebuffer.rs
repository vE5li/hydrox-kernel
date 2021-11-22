use peripherals::mailbox::{ Message, Channel, AlphaMode };

use super::font::*;

#[derive(Clone)]
pub struct Framebuffer {
    pub address: usize,
    pub pitch: usize,
    pub size: usize,
    pub width: usize,
    pub height: usize,
    pub bytes_per_pixel: usize,
}

impl Framebuffer {

    pub fn new(address: usize, pitch: usize, width: usize, height: usize, size: usize) -> Self {

        let bytes_per_pixel = size / height / width;
        log_line!("[ graphics ] monitor bytes per pixel: {}", bytes_per_pixel);

        return Self { address, pitch, size, width, height, bytes_per_pixel };
    }

    pub fn draw_pixel(&mut self, x_position: usize, y_position: usize, color: u32) {

        if self.bytes_per_pixel == 2 {

            let offset = (y_position * (self.width * 2)) + (x_position * 2);
            unsafe { *((self.address + offset) as *const u16 as *mut u16) = color as u16; }

        } else if self.bytes_per_pixel == 4 {

            let offset = (y_position * (self.width * 4)) + (x_position * 4);
            unsafe { *((self.address + offset) as *const u32 as *mut u32) = color; }

        } else {
            panic!("{} bytes per pixel is unsupported", self.bytes_per_pixel);
        }
    }

    pub fn draw_rectangle(&mut self, x_position: usize, y_position: usize, width: usize, height: usize, fill_color: u32, border_color: u32) {
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

    pub fn draw_character(&mut self, x_position: usize, y_position: usize, character: char, foreground_color: u32, backgroud_color: u32) {

        let glyph = &FONT[character as u8 as usize];

        for y in 0..FONT_HEIGHT {
            for x in 0..FONT_WIDTH {
                match glyph[y] & 1 << x == 0 {
                    true => self.draw_pixel(x_position + x, y_position + y, backgroud_color),
                    false => self.draw_pixel(x_position + x, y_position + y, foreground_color),
                }
            }
        }
    }

    pub fn draw_text(&mut self, mut x_position: usize, y_position: usize, text: &str, foreground_color: u32, backgroud_color: u32) {
        for byte in text.as_bytes().iter() { // use as bytes instead of chars for performance reasons
            self.draw_character(x_position, y_position, *byte as char, foreground_color, backgroud_color);
            x_position += FONT_WIDTH;
        }
    }
}

pub fn initialize() -> Framebuffer {

    let mut message = Message::<32>::new();
    message.get_framebuffer_size_request();
    message.finalize_send_receive(Channel::Tags);

    let framebuffer_size = message.get_framebuffer_size_response();
    log_line!("[ graphics ] monitor size: {}", framebuffer_size);

    //if width | height == 0 -> early return without creating a frame buffer

    let mut message = Message::<64>::new();
    message.set_physical_size_request(framebuffer_size);
    message.set_virtual_size_request(framebuffer_size);
    message.set_virtual_offset_request(0, 0);
    message.set_framebuffer_depth_request(16);
    message.set_alpha_mode_request(AlphaMode::Ignored);
    message.set_pixel_order_request(1);
    message.allocate_framebuffer_request(4096);
    message.get_framebuffer_pitch_request();
    message.finalize_send_receive(Channel::Tags);

    let depth = message.set_framebuffer_depth_response();
    let alpha_mode = message.set_alpha_mode_response();
    let pixel_order = message.set_pixel_order_response();
    let framebuffer_layout = message.allocate_framebuffer_response();
    let pitch = message.get_framebuffer_pitch_response();
    let address = framebuffer_layout.address as usize & 0x3FFFFFFF;

    log_line!("[ graphics ] framebuffer depth: {}", depth);
    log_line!("[ graphics ] framebuffer alpha mode: {:?}", alpha_mode);
    log_line!("[ graphics ] framebuffer pixel order: {}", pixel_order);
    log_line!("[ graphics ] framebuffer address: 0x{:x}", address);
    log_line!("[ graphics ] framebuffer size: {}", framebuffer_layout.size);
    log_line!("[ graphics ] framebuffer pitch: {}", pitch);

    let framebuffer = Framebuffer::new(address, pitch as usize, framebuffer_size.width as usize, framebuffer_size.height as usize, framebuffer_layout.size as usize);
    ::peripherals::logger::set_framebuffer(framebuffer.clone());

    success!("framebuffer initialized");

    return framebuffer;
}
