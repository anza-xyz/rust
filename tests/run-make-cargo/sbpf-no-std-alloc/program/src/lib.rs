// A `#![no_std]` program that links `core` and `alloc`, but not `std`. See `../rmake.rs`.
//
// The program succeeds if `entrypoint` returns 0. It uses the infallible allocation APIs, so
// it needs the default allocation error handler (`__rdl_alloc_error_handler`) from `alloc` at
// link time. A `std`-linked test binary never does, as `std` provides its own handler.

#![no_std]
#![feature(core_intrinsics)]
#![allow(internal_features)]

extern crate alloc;

use alloc::alloc::handle_alloc_error;
use alloc::boxed::Box;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use alloc::{format, vec};
use core::alloc::{GlobalAlloc, Layout};
use core::hint::black_box;

/// Start of the heap region in the Solana VM's memory map.
const HEAP_START: usize = 0x3_0000_0000;
/// Must not exceed the `--heap-size` the runner is invoked with.
const HEAP_LEN: usize = 1024 * 1024;

/// Bump allocator that hands out memory from the top of the VM heap downwards.
///
/// The first word of the heap (zero-initialized by the VM) holds the current position.
struct BumpAllocator;

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            let pos_ptr = HEAP_START as *mut usize;
            let mut pos = *pos_ptr;
            if pos == 0 {
                pos = HEAP_START + HEAP_LEN;
            }
            pos = pos.saturating_sub(layout.size());
            pos &= !(layout.align().saturating_sub(1));
            if pos < HEAP_START + core::mem::size_of::<usize>() {
                return core::ptr::null_mut();
            }
            *pos_ptr = pos;
            pos as *mut u8
        }
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {}
}

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    // Looping here would hang the runner, as it has an effectively unlimited compute budget.
    core::intrinsics::abort()
}

trait Shape {
    fn area(&self) -> u64;
}

struct Rect(u64, u64);

impl Shape for Rect {
    fn area(&self) -> u64 {
        self.0 * self.1
    }
}

fn run() {
    // Make sure `alloc`'s default allocation error handler is referenced from live code,
    // regardless of what the optimizer does with the infallible APIs below.
    if black_box(false) {
        handle_alloc_error(Layout::new::<u64>());
    }

    let mut v = Vec::new();
    for i in (0..1000u32).rev() {
        v.push(i);
    }
    v.sort();
    assert_eq!(v[0], 0);
    assert_eq!(v[999], 999);

    let s = format!("{}-{:?}", 42, "x");
    assert_eq!(s, "42-\"x\"");
    let mut owned: String = "abc".to_string();
    owned.push_str("def");
    assert_eq!(owned, "abcdef");

    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Rect(2, 3)), Box::new(Rect(4, 5))];
    assert_eq!(shapes.iter().map(|s| s.area()).sum::<u64>(), 26);

    let mut map = BTreeMap::new();
    for i in 0..100u64 {
        map.insert(i, i * i);
    }
    assert_eq!(map.get(&9), Some(&81));

    let mut deque = VecDeque::new();
    deque.push_back(1);
    deque.push_front(0);
    assert_eq!(deque.iter().copied().collect::<Vec<_>>(), [0, 1]);

    let rc = Rc::new(5);
    let rc2 = Rc::clone(&rc);
    assert_eq!(Rc::strong_count(&rc), 2);
    drop(rc2);
    assert_eq!(Rc::strong_count(&rc), 1);

    let arc = Arc::new(String::from("arc"));
    assert_eq!(*Arc::clone(&arc), "arc");

    // An allocation that cannot be satisfied is reported through the fallible API.
    let mut huge: Vec<u8> = Vec::new();
    assert!(huge.try_reserve(HEAP_LEN * 2).is_err());
}

#[unsafe(no_mangle)]
pub extern "C" fn entrypoint(_input: *mut u8) -> u64 {
    run();
    0
}
