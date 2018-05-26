use alloc::heap::{ GlobalAlloc, Layout, Opaque };
use core::sync::atomic::{ AtomicUsize, Ordering };

// heap allocator
pub struct Allocator {
    next:   AtomicUsize,
    base:   usize,
    limit:  usize,
}

// implement allocator
impl Allocator {

    // new instance
    pub const fn new(base: usize, size: usize) -> Self {
        Self {
            next:   AtomicUsize::new(base),
            base:   base,
            limit:  base + size,
        }
    }
}

// impolement global allocator for the heap allocator
unsafe impl GlobalAlloc for Allocator {

    // allocate memory on the heap
    unsafe fn alloc(&self, layout: Layout) -> *mut Opaque {
        let current = self.next.load(Ordering::Relaxed); // TODO: replace with swap
        let base = align_up(current, layout.align());
        let limit = base.saturating_add(layout.size());

        // assert bounds
        assert!(limit <= self.limit, "heap out of memory");
        self.next.store(limit, Ordering::Relaxed); // TODO: replace with swap
        base as *mut Opaque
    }

    // deallocate memory
    unsafe fn dealloc(&self, ptr: *mut Opaque, layout: Layout) {}
}

// align address down to fit alignment
pub fn align_down(address: usize, alignment: usize) -> usize {
    if alignment == 0 {
        address
    } else {
        assert!(alignment.is_power_of_two(), "invalid alignment");
        address & !(alignment - 1)
    }
}

// align address up
pub fn align_up(address: usize, alignment: usize) -> usize {
    align_down(address + alignment - 1, alignment)
}
