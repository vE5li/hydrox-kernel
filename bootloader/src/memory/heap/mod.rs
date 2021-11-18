use alloc::alloc::{ GlobalAlloc, Layout };
use core::sync::atomic::{ AtomicUsize, Ordering };

pub struct Allocator {
    next:   AtomicUsize,
    base:   usize,
    limit:  usize,
}

impl Allocator {

    pub const fn new(base: usize, size: usize) -> Self {
        return Self {
            next:   AtomicUsize::new(base),
            base:   base,
            limit:  base + size,
        };
    }
}

unsafe impl GlobalAlloc for Allocator {

    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let current = self.next.load(Ordering::Relaxed); // TODO: replace with swap
        let base = align_up(current, layout.align());
        let limit = base.saturating_add(layout.size());

        assert!(limit <= self.limit, "heap out of memory");
        self.next.store(limit, Ordering::Relaxed); // TODO: replace with swap
        base as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

pub fn align_down(address: usize, alignment: usize) -> usize {
    if alignment == 0 {
        address
    } else {
        assert!(alignment.is_power_of_two(), "invalid alignment");
        address & !(alignment - 1)
    }
}

pub fn align_up(address: usize, alignment: usize) -> usize {
    align_down(address + alignment - 1, alignment)
}
