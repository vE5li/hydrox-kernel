// convert a bus address to a physical address
macro_rules! bus_physical {
    ($base:expr)    => ($base & 0x3fffffff);
}

pub mod heap;

// copy memory
#[no_mangle]
pub extern fn memcpy(destination: *mut u8, source: *const u8, length: usize) -> *mut u8 {
    let mut offset = 0;
    while offset < length {
        unsafe { *destination.offset(offset as isize) = *source.offset(offset as isize) };
        offset += 1;
    }
    return destination;
}

// move memory
#[no_mangle]
pub extern fn memmove(destination: *mut u8, source: *const u8, length: usize) -> *mut u8 {
    if source < destination as *const u8 {
        let mut offset = length;
        while offset != 0 {
            offset -= 1;
            unsafe { *destination.offset(offset as isize) = *source.offset(offset as isize) };
        }
    } else {
        let mut offset = 0;
        while offset < length {
            unsafe { *destination.offset(offset as isize) = *source.offset(offset as isize) };
            offset += 1;
        }
    }
    return destination;
}

// set memory
#[no_mangle]
pub extern fn memset(destination: *mut u8, source: i32, length: usize) -> *mut u8 {
    let mut offset = 0;
    while offset < length {
        unsafe { *destination.offset(offset as isize) = source as u8 };
        offset += 1;
    }
    return destination;
}

// compare memory
#[no_mangle]
pub extern fn memcmp(source1: *const u8, source2: *const u8, length: usize) -> i32 {
    let mut offset = 0;
    while offset < length {
        let source1 = unsafe { *source1.offset(offset as isize) };
        let source2 = unsafe { *source2.offset(offset as isize) };
        if source1 != source2 {
            return source1 as i32 - source2 as i32
        }
        offset += 1;
    }
    return 0;
}
