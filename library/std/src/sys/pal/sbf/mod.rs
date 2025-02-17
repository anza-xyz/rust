//! System bindings for the SBF platform
//!
//! This module contains the facade (aka platform-specific) implementations of
//! OS level functionality for SBF
//!
//! This is all super highly experimental and not actually intended for
//! wide/production use yet, it's still all in the experimental category. This
//! will likely change over time.
//!
//! Currently all functions here are basically stubs that immediately return
//! errors. The hope is that with a portability lint we can turn actually just
//! remove all this and just omit parts of the standard library if we're
//! compiling for SBF. That way it's a compile time error for something that's
//! guaranteed to be a runtime error!

pub mod alloc;
pub mod args;
//#[cfg(feature = "backtrace")]
//pub mod backtrace;
pub mod env;
pub mod fs;
pub mod io;
pub mod net;
pub mod os;
#[path = "../unsupported/pipe.rs"]
pub mod pipe;
#[path = "../unsupported/process.rs"]
pub mod process;
pub mod stdio;
pub mod thread;
#[path = "../unsupported/thread_local_dtor.rs"]
pub mod thread_local_dtor;
#[path = "../unsupported/thread_local_key.rs"]
pub mod thread_local_key;
#[path = "../unsupported/thread_parking.rs"]
pub mod thread_parking;
pub mod time;


#[cfg(not(target_feature = "static-syscalls"))]
extern "C" {
    fn abort() -> !;
    fn sol_log_(message: *const u8, length: u64);
}

#[no_mangle]
#[allow(improper_ctypes)]
#[linkage = "weak"]
extern "C" fn custom_panic(_info: &core::panic::PanicInfo<'_>) {}

#[cfg(target_feature = "static-syscalls")]
#[inline(never)]
#[no_mangle]
#[link_section = ".text.abort"]
#[linkage = "external"]
pub unsafe extern "C" fn abort() -> ! {
    let syscall: extern "C" fn() -> ! = core::mem::transmute(3069975057u64); // murmur32 hash of "abort"
    syscall()
}

#[cfg(target_feature = "static-syscalls")]
unsafe extern "C" fn sol_log_(message: *const u8, length: u64) {
    let syscall: extern "C" fn(*const u8, u64) = core::mem::transmute(544561597u64); // murmur32 hash of "sol_log_"
    syscall(message, length)
}

pub fn sol_log(message: &[u8]) {
    unsafe {
        sol_log_(message.as_ptr(), message.len() as u64);
    }
}

pub fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    unsafe {
        custom_panic(info);
        abort();
    }
}

pub fn unsupported<T>() -> crate::io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> crate::io::Error {
    crate::io::Error::new(crate::io::ErrorKind::Other, "operation not supported on SBF yet")
}

pub fn decode_error_kind(_code: i32) -> crate::io::ErrorKind {
    crate::io::ErrorKind::Other
}

// This enum is used as the storage for a bunch of types which can't actually
// exist.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Void {}

pub fn abort_internal() -> ! {
    unsafe { abort() }
}

// We don't have randomness yet, but I totally used a random number generator to
// generate these numbers.
//
// More seriously though this is just for DOS protection in hash maps. It's ok
// if we don't do that on SBF just yet.
pub fn hashmap_random_keys() -> (u64, u64) {
    (1, 2)
}

#[inline]
pub fn is_interrupted(_errno: i32) -> bool {
    false
}
