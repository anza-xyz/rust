// Trying to satisfy clippy here is hopeless
#![allow(clippy::style)]
// FIXME(e2024): this eventually needs to be removed.
#![allow(unsafe_op_in_unsafe_fn)]

// memcpy/memmove/memset have optimized implementations on some architectures
#[cfg(not(target_os = "solana"))]
#[cfg_attr(
    all(not(feature = "no-asm"), target_arch = "x86_64"),
    path = "x86_64.rs"
)]
mod impls;

#[cfg(not(target_os = "solana"))]
intrinsics! {
    #[mem_builtin]
    pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
        impls::copy_forward(dest, src, n);
        dest
    }

    #[mem_builtin]
    pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
        let delta = (dest as usize).wrapping_sub(src as usize);
        if delta >= n {
            // We can copy forwards because either dest is far enough ahead of src,
            // or src is ahead of dest (and delta overflowed).
            impls::copy_forward(dest, src, n);
        } else {
            impls::copy_backward(dest, src, n);
        }
        dest
    }

    #[mem_builtin]
    pub unsafe extern "C" fn memset(s: *mut u8, c: core::ffi::c_int, n: usize) -> *mut u8 {
        impls::set_bytes(s, c as u8, n);
        s
    }

    #[mem_builtin]
    pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> core::ffi::c_int {
        impls::compare_bytes(s1, s2, n)
    }

    #[mem_builtin]
    pub unsafe extern "C" fn bcmp(s1: *const u8, s2: *const u8, n: usize) -> core::ffi::c_int {
        memcmp(s1, s2, n)
    }

    #[mem_builtin]
    pub unsafe extern "C" fn strlen(s: *const core::ffi::c_char) -> usize {
        impls::c_string_length(s)
    }
}

// MEM functions have been rewritten to copy 8 byte chunks.  No
// compensation for alignment is made here with the requirement that
// the underlying hardware supports unaligned loads/stores.  If the
// number of store operations is greater than 8 the memory operation
// is performed in the run-time system instead, by calling the
// corresponding "C" function.
#[cfg(all(target_os = "solana", not(target_feature = "static-syscalls")))]
mod syscalls {
    unsafe extern "C" {
        pub fn sol_memcpy_(dest: *mut u8, src: *const u8, n: u64);
        pub fn sol_memmove_(dest: *mut u8, src: *const u8, n: u64);
        pub fn sol_memset_(s: *mut u8, c: u8, n: u64);
        pub fn sol_memcmp_(s1: *const u8, s2: *const u8, n: u64, result: *mut i32);
    }
}

#[cfg(all(target_os = "solana", target_feature = "static-syscalls"))]
mod syscalls {

    #[inline]
    pub(crate) fn sol_memcpy_(dest: *mut u8, src: *const u8, n: u64) {
        let syscall: extern "C" fn(*mut u8, *const u8, u64) =
            unsafe { core::mem::transmute(1904002211u64) }; // murmur32 hash of "sol_memcpy_"
        syscall(dest, src, n)
    }

    #[inline]
    pub(crate) fn sol_memmove_(dest: *mut u8, src: *const u8, n: u64) {
        let syscall: extern "C" fn(*mut u8, *const u8, u64) =
            unsafe { core::mem::transmute(1128493560u64) }; // murmur32 hash of "sol_memmove_"
        syscall(dest, src, n)
    }

    #[inline]
    pub(crate) fn sol_memcmp_(dest: *const u8, src: *const u8, n: u64, result: *mut i32) {
        let syscall: extern "C" fn(*const u8, *const u8, u64, *mut i32) =
            unsafe { core::mem::transmute(1608310321u64) }; // murmur32 hash of "sol_memcmp_"
        syscall(dest, src, n, result)
    }

    #[inline]
    pub(crate) fn sol_memset_(dest: *mut u8, c: u8, n: u64) {
        let syscall: extern "C" fn(*mut u8, u8, u64) =
            unsafe { core::mem::transmute(930151202u64) }; // murmur32 hash of "sol_memset_"
        syscall(dest, c, n)
    }
}

#[cfg(target_os = "solana")]
use self::syscalls::*;

#[cfg(target_os = "solana")]
#[cfg_attr(
    all(feature = "mem-unaligned", not(feature = "mangled-names")),
    unsafe(no_mangle)
)]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    sol_memcpy_(dest, src, n as u64);
    dest
}

#[cfg(target_os = "solana")]
#[cfg_attr(
    all(feature = "mem-unaligned", not(feature = "mangled-names")),
    unsafe(no_mangle)
)]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    sol_memmove_(dest, src, n as u64);
    dest
}

#[cfg(target_os = "solana")]
#[cfg_attr(
    all(feature = "mem-unaligned", not(feature = "mangled-names")),
    unsafe(no_mangle)
)]
pub unsafe extern "C" fn memset(s: *mut u8, c: c_int, n: usize) -> *mut u8 {
    sol_memset_(s, c as u8, n as u64);
    s
}

#[cfg(target_os = "solana")]
#[cfg_attr(
    all(feature = "mem-unaligned", not(feature = "mangled-names")),
    unsafe(no_mangle)
)]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut result = 0;
    sol_memcmp_(s1, s2, n as u64, &mut result as *mut i32);
    result
}
