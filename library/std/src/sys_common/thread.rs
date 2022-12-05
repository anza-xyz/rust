#[cfg(not(target_family = "solana"))]
use crate::env;
#[cfg(not(target_family = "solana"))]
use crate::sync::atomic::{self, Ordering};
#[cfg(not(target_family = "solana"))]
use crate::sys::thread as imp;

#[cfg(not(target_family = "solana"))]
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

#[cfg(target_family = "solana")]
pub fn min_stack() -> usize {
    0
}
