//! This is an implementation of a global allocator on the SBF platform.
//! In that situation there's no actual runtime for us
//! to lean on for allocation, so instead we provide our own!
//!
//! The crate itself provides a global allocator which on SBF has no
//! synchronization as there are no threads!

use crate::alloc::{GlobalAlloc, Layout, System};
use crate::ptr;

/// Alignment guaranteed by `sol_alloc_free_` (`BPF_ALIGN_OF_U128` in the runtime). The syscall
/// only receives a size, so larger alignments must be handled here.
const MIN_ALIGN: usize = 8;

#[inline]
unsafe fn aligned_alloc(layout: Layout) -> *mut u8 {
    if layout.align() <= MIN_ALIGN {
        return sol_alloc_free_(layout.size() as u64, 0);
    }
    // Over-allocate and round the pointer up. The runtime never frees memory, so the padding is
    // not wasted any more than the allocation itself would be.
    let Some(size) = layout.size().checked_add(layout.align() - 1) else {
        return ptr::null_mut();
    };
    let ptr = sol_alloc_free_(size as u64, 0);
    if ptr.is_null() {
        return ptr;
    }
    ptr.map_addr(|addr| (addr + layout.align() - 1) & !(layout.align() - 1))
}

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        aligned_alloc(layout)
    }

    // Fresh memory from `sol_alloc_free_` is always zeroed, since the runtime never reuses it.
    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        aligned_alloc(layout)
    }

    // The runtime does not implement freeing, so passing a pointer that `aligned_alloc` rounded
    // up (rather than the one the syscall returned) is harmless.
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

#[cfg(not(target_feature = "static-syscalls"))]
unsafe extern "C" {
    fn sol_alloc_free_(size: u64, ptr: u64) -> *mut u8;
}

#[cfg(target_feature = "static-syscalls")]
fn sol_alloc_free_(size: u64, ptr: u64) -> *mut u8 {
    let syscall: extern "C" fn(u64, u64) -> *mut u8 =
        unsafe { core::mem::transmute(2213547663u64) }; // murmur32 hash of "sol_alloc_free_"
    syscall(size, ptr)
}
