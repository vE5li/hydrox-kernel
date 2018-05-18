use ::peripherals::mailman::{ Letter, Channel, MailboxTag };

// all the information needed to render to the frambuffer
pub struct FrameBuffer {
    pub base:   usize,
    pub pitch:  usize,
    pub width:  u32,
    pub height: u32,
}

// initialize a framebuffer
pub fn initialize() -> FrameBuffer {

    // get the frambuffer size
    let mut letter = Letter::new();
    letter.push_tag(MailboxTag::GetFramebufferSize, &[0, 0]);
    letter.push_end_tag();
    letter.send(Channel::Tags);
    letter.receive(Channel::Tags);

    // width and height
    let (width, height) = {
        let buffer = letter.buffer();
        (buffer[5], buffer[6])
    };

    // set the frambuffer size and depth, then allocate it
    letter.clear();
    letter.push_tag(MailboxTag::SetPhysicalSize, &[width, height]);
    letter.push_tag(MailboxTag::SetVirtualSize, &[width, height]);
    letter.push_tag(MailboxTag::SetFramebufferDepth, &[16]);
    letter.push_tag(MailboxTag::AllocateFramebuffer, &[16, 0]);
    letter.push_end_tag();
    letter.send(Channel::Tags);
    letter.receive(Channel::Tags);

    // framebuffer base
    let base = {
        let index = 3 + letter.tag_index(MailboxTag::AllocateFramebuffer).expect("invalid allocate frambuffer response");
        bus_physical!(letter.buffer()[index] as usize)
    };

    // request the framebuffer base
    letter.clear();
    letter.push_tag(MailboxTag::GetFramebufferPitch, &[0]);
    letter.push_end_tag();
    letter.send(Channel::Tags);
    letter.receive(Channel::Tags);

    // frambuffer pitch
    let pitch = letter.buffer()[5] as usize;

    // create the framebuffer structure
    FrameBuffer {
        base: base,
        pitch: pitch,
        width: width,
        height: height,
    }
}
