pub use std::io::{ Result, Read, Write };
pub use std::fs::File;
use std::env;
use std::fs;

#[cfg(feature = "input")]
pub use input::*;

// version channels
#[derive(Copy, Clone)]
pub enum Channel {
    Stable,
    Beta,
    Nightly,
    None,
}

// implement channel
impl Channel {

    // get the channel from a string
    pub fn from_raw(source: &str) -> Channel {
        match source {
            "stable" => Channel::Stable,
            "beta" => Channel::Beta,
            "nightly" => Channel::Nightly,
            "none" => Channel::None,
            channel => panic!("[ loader ] [ config ] no channel named '{}'. must be 'stable', 'beta', 'nightly' or 'none'", channel),
        }
    }

    // get an extention from the channel
    pub fn suffix(&self) -> &'static str {
        match *self {
            Channel::Stable => ".stable",
            Channel::Beta => ".beta",
            Channel::Nightly => ".nightly",
            Channel::None => "",
        }
    }
}

// possible loader modes
pub enum LoadMode {
    Listed(Vec<(u32, usize, Channel)>),
    Indexed(usize),
}

// holds the command line arguments
pub struct Context {
    #[cfg(feature = "input")]
    pub translation_path:   String,
    pub serial_device_path: String,
    pub images:             Vec<String>,
    pub channel:            Channel,
    pub load_mode:          LoadMode,
    pub instant_load:       bool,
}

// get the size of a file in bytes
pub fn file_size(filename: &str) -> Result<u32> {
    let metadata = fs::metadata(filename)?;
    Ok(metadata.len() as u32)
}

// parse the config file
fn parse_config(context: &mut Context, filename: &str, prefix: &mut String) {
    use std::io::{BufRead, BufReader};

    // attempt to open the config file
    let reader = BufReader::new(File::open(filename).expect(&format!("[ loader ] unable to open specified config file '{}'", filename)));

    // loop through every line and parse it unless it's commented
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.len() == 0 || line.chars().nth(0).unwrap() == '#' {
                continue;
            }

            // get a "stack" of words from the line and pop all values
            let mut words: Vec<&str> = line.split_whitespace().rev().collect();
            while let Some(word) = words.pop() {
                match word {

                    // specify a file for input to be translated from
                    #[cfg(feature = "input")]
                    ":translate" => context.translation_path = words.pop().expect("[ loader ] [ config ] no translation file specified").to_string(),

                    // add a kernel image
                    ":image" => context.images.push(prefix.clone() + &words.pop().expect("[ loader ] [ config ] no binary filename specified")),

                    // parse a configuration file
                    ":config" => parse_config(context, &words.pop().expect("[ loader ] [ config ] no config file specified"), prefix),

                    // set serial device path
                    ":serial" => context.serial_device_path = words.pop().expect("[ loader ] [ config ] no serial device specified").to_string(),

                    // set a prefix for the kernel images to be added
                    ":prefix" => *prefix = words.pop().expect("[ loader ] [ config ] no prefix specified").to_string(),

                    // specify index for choosing a kernel image
                    ":use" => context.load_mode = LoadMode::Indexed(words.pop().expect("[ loader ] [ config ] no index for use specified").parse().expect("[ loader ] [ config ] unable to parse index to use")),

                    // specify the channel to load from
                    ":channel" => context.channel = Channel::from_raw(words.pop().expect("[ loader ] [ config ] no channel specified")),

                    // set instant load
                    ":instant" => context.instant_load = match words.pop().expect("[ loader ] [ config ] no value for instant-load specified") {
                        "enable" => true,
                        "disable" => false,
                        value => panic!("[ loader ] [ config ] undefined instant-load value '{}'. must be 'enable' or 'disable'", value),
                    },

                    // assign a certain ip to a load index
                    ":assign" => {
                        let address: u32 = words.pop().expect("[ loader ] [ config ] no ethernet address specified").parse().expect("[ loader ] [ config ] unable to parse address");
                        let index: usize = words.pop().expect("[ loader ] [ config ] no image index specified").parse().expect("[ loader ] [ config ] unable to parse index");
                        let channel = match words.pop() {
                            Some(word) => Channel::from_raw(word),
                            None => context.channel,
                        };

                        // if the mode is already listed, push the element. otherwise set the mode to listed
                        if let LoadMode::Listed(list) = &mut context.load_mode {
                            list.push((address, index, channel));
                            continue;
                        }
                        context.load_mode = LoadMode::Listed(vec!((address, index, channel)));
                    },

                    // invalid setting
                    _ => panic!("[ loader ] [ config ] unsupported setting '{}'", word),
                }
            }
        }
    }
}

// parse the command line arguments
pub fn parse_parameters() -> Context {

    // needed only for parsing
    let mut prefix = String::new();

    // collect parameters and create a clean context
    let mut parameters: Vec<String> = env::args().rev().collect();
    let mut context = Context {
        #[cfg(feature = "input")]
        translation_path:       String::new(),
        images:                 Vec::new(),
        channel:                Channel::None,
        load_mode:              LoadMode::Indexed(0),
        serial_device_path:     String::new(),
        instant_load:           false,
    };

    // pop loader path
    parameters.pop();

    // iterate over all words
    while let Some(word) = parameters.pop() {
        if word.chars().nth(0).unwrap() == '-' {
            match word.as_ref() {

                // specify a file for input to be translated from
                #[cfg(feature = "input")]
                "-t" => context.translation_path = parameters.pop().expect("[ loader ] [ flag ] no translation file specified").to_string(),

                // parse a configuration file
                "-c" => parse_config(&mut context, &parameters.pop().expect("[ loader ] [ flag ] no config file specified"), &mut prefix),

                // set serial device path
                "-s" => context.serial_device_path = parameters.pop().expect("[ loader ] [ flag ] no serial device specified").to_string(),

                // set a prefix for the kernel images to be added
                "-p" => prefix = parameters.pop().expect("[ loader ] [ flag ] no prefix specified"),

                // specify index for choosing a kernel image
                "-u" => context.load_mode = LoadMode::Indexed(parameters.pop().expect("[ loader ] [ flag ] no index for use specified").parse().expect("[ loader ] [ flag ] unable to parse index to use")),

                // specify the channel to load from
                "-r" => context.channel = Channel::from_raw(&parameters.pop().expect("[ loader ] [ config ] no channel specified")),

                // enable instant load
                "-i" => context.instant_load = true,

                // invalid flag
                _ => panic!("[ loader ] invalid flag '{}'", word),
            }
        } else {
            context.images.push(prefix.clone() + &word);
        }
    }

    // assert there is at least one kernel image specified
    assert!(context.images.len() != 0, "[ loader ] no binary files specified");
    context
}
