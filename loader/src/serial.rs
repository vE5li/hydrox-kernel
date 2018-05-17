#[cfg(feature = "input")]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(feature = "input")]
use std::sync::Arc;

use common::*;

// start the keyboard daemon
#[cfg(feature = "input")]
fn start_daemon(context: &Context) -> Arc<AtomicBool> {

    // clone strings for passing
    let loading = Arc::new(AtomicBool::new(false));
    let loading_clone = loading.clone();
    let serial_device_path = context.serial_device_path.clone();
    let translation_path = context.translation_path.clone();

    // spawn the input thread
    thread::spawn(|| { daemon(serial_device_path, translation_path, loading_clone) });
    loading
}

// keyboard input daemon
#[cfg(feature = "input")]
fn daemon(serial_device_path: String, translation_path: String, loading: Arc<AtomicBool>) {

    // create an input handler and open the serial device
    let mut input_handler = InputHandler::new(&translation_path);
    let mut serial_device = File::create(serial_device_path).expect("[ loader ] unable to open serial device");

    // success
    println!("[ loader ] serial input daemon started");

    // wait for a new event to write over serial
    loop {
        let (modifiers, keycode) = input_handler.read_event();
        if loading.load(Ordering::Relaxed) == false {
            serial_device.write(&[modifiers, keycode]).unwrap();
        }
    }
}

// transmit a 32 bit number
fn transmit_size(serial_device: &mut File, size: u32) -> Result<()> {
    let buffer: [u8; 4] = [(size >> 24) as u8, (size >> 16) as u8, (size >> 8) as u8, size as u8];
    serial_device.write_all(&buffer)
}

// transmit a binary file
fn transmit_binary(serial_device_path: &str, filename: &str) {
    use std::time::Instant;

    // open serial device and kernel binary file
    let mut serial_device = File::create(serial_device_path).expect("[ loader ] unable to open serial device");
    if let Ok(binary) = File::open(filename) {
        let binary_size = file_size(filename).expect("[ loader ] unable to read binary file size");

        // get current time
        let time = Instant::now();

        // send initial transmition character and send the size
        println!("[ loader ] transmitting 0x{:x} byte/s", binary_size);
        serial_device.write(&[b'!']).unwrap();
        transmit_size(&mut serial_device, binary_size).expect("[ loader ] unable to send binary file size");

        // iterate over every byte in the binary and send it
        for byte in binary.bytes() {
            serial_device.write(&[byte.unwrap()]).unwrap();
        }
        println!("[ loader ] transmitted in {} second/s", time.elapsed().as_secs());
    } else {
        println!("[ loader ] unable to open binary file {}", filename);
    }
}

// wait till a request is received
fn idle(serial_device_path: &str) {

    // open serial device
    let mut serial_device = File::open(serial_device_path).expect("[ loader ] unable to open serial device");
    let mut buffer = [0];

    // read incoming characters and break for transmition to start
    loop {
        serial_device.read_exact(&mut buffer).unwrap();
        if buffer[0] == b'?' {
            break;
        }
        print!("{}", buffer[0] as char);
    }
}

// start the serial mode
pub fn start(context: Context) {
    if let LoadMode::Indexed(index) = context.load_mode {
        let image = context.images[index].clone() + context.channel.suffix();
        println!("[ loader ] starting in serial mode");
        println!("[ loader ] selected kernel image '{}'", &image);

        // check if instant load in enabled
        if context.instant_load {
            println!("[ loader ] instant load enabled");
            transmit_binary(&context.serial_device_path, &image);
        } else {
            #[cfg(feature = "input")]
            let loading = start_daemon(&context);
            loop {
                idle(&context.serial_device_path);

                // block input
                #[cfg(feature = "input")]
                loading.store(true, Ordering::Relaxed);

                // send the kernel
                transmit_binary(&context.serial_device_path, &image);

                // unblock input
                #[cfg(feature = "input")]
                loading.store(false, Ordering::Relaxed);
            }
        }
    } else {
        panic!("[ loader ] only single images are supported in serial mode");
    }
}
