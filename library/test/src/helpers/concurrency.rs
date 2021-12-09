//! Helper module which helps to determine amount of threads to be used
//! during tests execution.
#[cfg(all(not(target_arch = "bpf"), not(target_arch = "sbf")))]
use std::{env, num::NonZeroUsize, thread};

#[cfg(all(not(target_arch = "bpf"), not(target_arch = "sbf")))]
pub fn get_concurrency() -> usize {
    if let Ok(value) = env::var("RUST_TEST_THREADS") {
        match value.parse::<NonZeroUsize>().ok() {
            Some(n) => n.get(),
            _ => panic!("RUST_TEST_THREADS is `{value}`, should be a positive integer."),
        }
    } else {
        thread::available_parallelism().map(|n| n.get()).unwrap_or(1)
    }
}

#[cfg(any(target_arch = "bpf", target_arch = "sbf"))]
pub fn get_concurrency() -> usize {
    1
}
