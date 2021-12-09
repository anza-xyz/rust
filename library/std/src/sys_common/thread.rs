#[cfg(all(not(target_arch = "bpf"), not(target_arch = "sbf")))]
use crate::env;
#[cfg(all(not(target_arch = "bpf"), not(target_arch = "sbf")))]
use crate::sync::atomic::{self, Ordering};
#[cfg(all(not(target_arch = "bpf"), not(target_arch = "sbf")))]
use crate::sys::thread as imp;

#[cfg(all(not(target_arch = "bpf"), not(target_arch = "sbf")))]
pub fn min_stack() -> usize {
    static MIN: atomic::AtomicUsize = atomic::AtomicUsize::new(0);
    match MIN.load(Ordering::Relaxed) {
        0 => {}
        n => return n - 1,
    }
    let amt = env::var("RUST_MIN_STACK").ok().and_then(|s| s.parse().ok());
    let amt = amt.unwrap_or(imp::DEFAULT_MIN_STACK_SIZE);

    // 0 is our sentinel value, so ensure that we'll never see 0 after
    // initialization has run
    MIN.store(amt + 1, Ordering::Relaxed);
    amt
}

#[cfg(any(target_arch = "bpf", target_arch = "sbf"))]
pub fn min_stack() -> usize {
    0
}
