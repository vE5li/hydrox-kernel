use ::peripherals::mailman;

// all the information needed to render to the frambuffer
pub struct FrameBuffer {
    pub base:   usize,
    pub pitch:  usize,
    pub width:  u32,
    pub height: u32,
}

// initialize a framebuffer
pub fn initialize() -> FrameBuffer {
    // request the framebuffer width and height
    mailman::push_tag(&[0x40003, 8, 0, 0, 0]);
    let buffer = mailman::pop_tag();
    let width = buffer[5];
    let height = buffer[6];

    // request the framebuffer base
    mailman::push_tag(&[0x48003, 8, 8, width, height, 0x48004, 8, 8, width, height, 0x48005, 4, 4, 16, 0x40001, 8, 4, 16, 0]);
    let buffer = mailman::pop_tag();
    let mut index = 2;
    while buffer[index] != 0x40001 {
        index += 3 + (buffer[index + 1] >> 2) as usize;
    }
    let base = bus_physical!(buffer[index + 3] as usize);

    // request the framebuffer pitch
    mailman::push_tag(&[0x40008, 4, 0, 0]);
    let pitch = mailman::pop_tag()[5] as usize;

    // create the framebuffer structure
    FrameBuffer {
        base: base,
        pitch: pitch,
        width: width,
        height: height,
    }
}
