//! This is an implementation of a global allocator on the BPF platform.
//! In that situation there's no actual runtime for us
//! to lean on for allocation, so instead we provide our own!
//!
//! The crate itself provides a global allocator which on BPF has no
//! synchronization as there are no threads!

use crate::alloc::{GlobalAlloc, Layout, System};

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        sol_alloc_free_(layout.size() as u64, 0)
        // 0 as *mut u8
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        sol_alloc_free_(layout.size() as u64, 0)
        // 0 as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        sol_alloc_free_(layout.size() as u64, ptr as u64);
    }

    // #[inline]
    // unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
    //     sol_alloc_free_(layout.size() as u64, 0)
    //     // 0 as *mut u8
    // }
}
extern "C" {
    fn sol_alloc_free_(size: u64, ptr: u64) -> *mut u8;
}
