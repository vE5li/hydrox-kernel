pub mod font;
pub mod framebuffer;

pub use self::font::*;
pub use self::framebuffer::Framebuffer;

pub fn initialize() -> Framebuffer {

    let framebuffer = framebuffer::initialize();

    success!("graphics initialized");

    return framebuffer;
}
