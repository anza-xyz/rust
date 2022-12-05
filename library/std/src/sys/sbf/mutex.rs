use crate::cell::UnsafeCell;
use crate::sys_common::lazy_box::{LazyBox, LazyInit};

pub struct Mutex {
    inner: UnsafeCell<bool>,
}

pub(crate) type MovableMutex = LazyBox<Mutex>;

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {} // no threads on SBF

impl LazyInit for Mutex {
    fn init() -> Box<Self> {
        Box::new(Self::new())
    }
}

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
