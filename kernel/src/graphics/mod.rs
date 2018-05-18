pub mod framebuffer;

// initialize all graphics
pub fn initialize() {

    // get a framebuffer
    let framebuffer = framebuffer::initialize();
    log!("framebuffer base at 0x{:x}", framebuffer.base);
    unsafe { *(framebuffer.base as *mut u16) = 0xff0f };
}
