//! Helper module which helps to determine amount of threads to be used
//! during tests execution.
#[cfg(not(target_arch = "bpf"))]
use std::{env, num::NonZeroUsize, thread};

#[cfg(not(target_arch = "bpf"))]
pub fn get_concurrency() -> usize {
    if let Ok(value) = env::var("RUST_TEST_THREADS") {
        match value.parse::<NonZeroUsize>().ok() {
            Some(n) => n.get(),
            _ => panic!("RUST_TEST_THREADS is `{}`, should be a positive integer.", value),
        }
    } else {
        thread::available_concurrency().map(|n| n.get()).unwrap_or(1)
    }
}

#[cfg(target_arch = "bpf")]
pub fn get_concurrency() -> usize {
    1
}
