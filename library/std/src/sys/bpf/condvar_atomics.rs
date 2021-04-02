use crate::cmp;
use crate::mem;
use crate::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use crate::sys::mutex::Mutex;
use crate::time::Duration;

pub struct Condvar {
    cnt: AtomicUsize,
}

impl Condvar {
    pub const fn new() -> Condvar {
        Condvar { cnt: AtomicUsize::new(0) }
    }

    #[inline]
    pub unsafe fn init(&mut self) {
        // nothing to do...
    }

    pub unsafe fn notify_one(&self) {
        // nothing to do...
    }

    #[inline]
    pub unsafe fn notify_all(&self) {
        // nothing to do...
    }

    pub unsafe fn wait(&self, mutex: &Mutex) {
        // nothing to do...
    }

    pub unsafe fn wait_timeout(&self, mutex: &Mutex, dur: Duration) -> bool {
        true
    }

    #[inline]
    pub unsafe fn destroy(&self) {
        // nothing to do
    }

    #[inline]
    fn ptr(&self) -> *mut i32 {
        assert_eq!(mem::size_of::<usize>(), mem::size_of::<i32>());
        &self.cnt as *const AtomicUsize as *mut i32
    }
}
