use crate::cell::UnsafeCell;

pub struct Mutex {
    inner: UnsafeCell<bool>,
}

pub type MovableMutex = Box<Mutex>;

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {} // no threads on BPF

#[allow(dead_code)] // sys isn't exported yet
impl Mutex {
    pub const fn new() -> Mutex {
        Mutex { inner: UnsafeCell::new(false) }
    }
    #[inline]
    pub unsafe fn init(&self) {}
    #[inline]
    pub unsafe fn lock(&self) {
        let locked = self.inner.get();
        assert!(!*locked, "cannot recursively acquire mutex");
        *locked = true;
    }
    #[inline]
    pub unsafe fn unlock(&self) {
        *self.inner.get() = false;
    }
    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        let locked = self.inner.get();
        if *locked {
            false
        } else {
            *locked = true;
            true
        }
    }
    #[inline]
    pub unsafe fn destroy(&self) {
    }
}

// All empty stubs because BPF has no threads, lock acquisition always
// succeeds.
pub struct ReentrantMutex {
}

impl ReentrantMutex {
    pub unsafe fn unlock(&self) {}
    pub unsafe fn destroy(&self) {}
}
