use crate::spin_lock::PiSpinLockMutex;
use core::alloc::{GlobalAlloc, Layout};
use util::allocator::Allocator;

pub struct LockedAllocator(pub PiSpinLockMutex<Allocator, 0>);

impl LockedAllocator {
    pub const fn new() -> Self {
        Self(PiSpinLockMutex::new(Allocator::new()))
    }
}

unsafe impl GlobalAlloc for LockedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut lock = self.0.lock();
        lock.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut lock = self.0.lock();
        lock.dealloc(ptr, layout);
    }
}
