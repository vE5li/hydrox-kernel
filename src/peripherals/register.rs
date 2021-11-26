macro_rules! read_register {
    ($registers:expr, $field:ident) => ( unsafe { core::ptr::read_volatile(&(*$registers).$field) } );
    ($registers:expr, $field:ident, $index:expr) => ( unsafe { core::ptr::read_volatile(&(*$registers).$field[$index]) } );
}

macro_rules! write_register {
    ($registers:expr, $field:ident, $value:expr) => ( unsafe { core::ptr::write_volatile(&mut (*$registers).$field, $value); } );
    ($registers:expr, $field:ident, $index:expr, $value:expr) => ( unsafe { core::ptr::write_volatile(&mut (*$registers).$field[$index], $value); } );
}
