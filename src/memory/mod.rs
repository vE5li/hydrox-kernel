pub mod heap;

#[no_mangle]
pub extern fn memcpy(destination: *mut u8, source: *const u8, length: usize) -> *mut u8 {
    (0..length).for_each(|offset| unsafe { *destination.offset(offset as isize) = *source.offset(offset as isize) });
    return destination;
}

#[no_mangle]
pub extern fn memmove(destination: *mut u8, source: *const u8, length: usize) -> *mut u8 {
    match source < destination as *const u8 {
        true => (0..length).rev().for_each(|offset| unsafe { *destination.offset(offset as isize) = *source.offset(offset as isize) }),
        false => { memcpy(destination, source, length); },
    }
    return destination;
}

#[no_mangle]
pub extern fn memset(destination: *mut u8, source: i32, length: usize) -> *mut u8 {
    (0..length).for_each(|offset| unsafe { *destination.offset(offset as isize) = source as u8 });
    return destination;
}

#[no_mangle]
pub extern fn memcmp(source1: *const u8, source2: *const u8, length: usize) -> i32 {
    for offset in 0..length {
        let source1 = unsafe { *source1.offset(offset as isize) };
        let source2 = unsafe { *source2.offset(offset as isize) };
        if source1 != source2 {
            return source1 as i32 - source2 as i32
        }
    }
    return 0;
}
