// convert a bus address to a physical address
macro_rules! bus_physical {
    ($base:expr) => ($base & 0x3fffffff);
}
