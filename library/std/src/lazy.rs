#![cfg(all(not(target_arch = "bpf"), not(target_arch = "sbf")))]

//! Lazy values and one-time initialization of static data.
