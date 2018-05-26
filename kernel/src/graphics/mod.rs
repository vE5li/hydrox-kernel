pub mod framebuffer;

// initialize all graphics
pub fn initialize() {

    // get a framebuffer
    let framebuffer = framebuffer::initialize();

    success!("graphics initialized");
}
