mod common;
mod ethernet;
mod serial;

#[cfg(feature = "input")]
pub mod input;

// main
fn main() {

    // parsee command line arguments passed to the loader
    let context = common::parse_parameters();

    // if there is no serial device specified, start in ethernet mode
    if context.serial_device_path.len() == 0 {
        ethernet::start(context);
    } else {
        serial::start(context);
    }
}
