// use crate::arch::BPF;
use crate::cell::UnsafeCell;
use crate::mem;
use crate::sync::atomic::{AtomicUsize, AtomicU32, Ordering::SeqCst};
use crate::sys::thread;

pub struct Mutex {
    locked: AtomicUsize,
}

impl Mutex {
    pub const fn new() -> Mutex {
        Mutex { locked: AtomicUsize::new(0) }
    }

    #[inline]
    pub unsafe fn init(&mut self) {
        // nothing to do
    }

    pub unsafe fn lock(&self) {
        // nothing to do...
    }

    pub unsafe fn unlock(&self) {
        // nothing to do...
    }

    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        true
    }

    #[inline]
    pub unsafe fn destroy(&self) {
        // nothing to do
    }

    #[inline]
    fn ptr(&self) -> *mut i32 {
        assert_eq!(mem::size_of::<usize>(), mem::size_of::<i32>());
        &self.locked as *const AtomicUsize as *mut isize as *mut i32
    }
}

pub struct ReentrantMutex {
    owner: AtomicU32,
    recursions: UnsafeCell<u32>,
}

unsafe impl Send for ReentrantMutex {}
unsafe impl Sync for ReentrantMutex {}

impl ReentrantMutex {
    pub unsafe fn uninitialized() -> ReentrantMutex {
        ReentrantMutex {
            owner: AtomicU32::new(0),
            recursions: UnsafeCell::new(0),
        }
    }

    pub unsafe fn init(&mut self) {
        // nothing to do...
    }

    pub unsafe fn lock(&self) {
        // nothing to do...
    }

    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        // nothing to do...
    }

    #[inline]
    unsafe fn _try_lock(&self, id: u32) -> Result<(), u32> {
        Ok(())
    }

    pub unsafe fn unlock(&self) {
        // nothing to do...
    }

    pub unsafe fn destroy(&self) {
        // nothing to do...
    }

    #[inline]
    fn ptr(&self) -> *mut i32 {
        &self.owner as *const AtomicU32 as *mut i32
    }
}
