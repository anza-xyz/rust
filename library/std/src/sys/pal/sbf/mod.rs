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
#[path = "../unsupported/os.rs"]
pub mod os;
pub mod path {}
pub mod thread {}
#[path = "../unsupported/thread_local_dtor.rs"]
pub mod thread_local_dtor;

#[cfg(not(target_feature = "static-syscalls"))]
unsafe extern "C" {
    fn abort() -> !;
    fn sol_log_(message: *const u8, length: u64);
}

#[unsafe(no_mangle)]
#[allow(improper_ctypes)]
#[linkage = "weak"]
extern "C" fn custom_panic(_info: &core::panic::PanicInfo<'_>) {}

#[cfg(target_feature = "static-syscalls")]
#[inline(never)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.abort")]
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

pub fn abort_internal() -> ! {
    unsafe { abort() }
}
