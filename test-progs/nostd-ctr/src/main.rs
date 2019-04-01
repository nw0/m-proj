#![feature(lang_items)]
#![feature(optin_builtin_traits)]
#![feature(on_unimplemented)]
#![feature(alloc)]
#![feature(alloc_error_handler)]

#![no_std]

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::option::Option::{self,*};
use core::iter::Iterator;

use alloc::boxed::Box;

#[cfg(target_os = "freebsd")]
#[link(name = "c")]
extern {
    pub fn printf(format: *const u8, ...) -> i32;
    pub fn malloc(size: usize) -> *mut u8;
    pub fn free(ptr: *mut u8);
}

#[lang = "start"]
fn start(_main: *const u8, _argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}

struct HeapAlloc;

unsafe impl GlobalAlloc for HeapAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size())
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        free(ptr)
    }
}

#[global_allocator]
static HEAP: HeapAlloc = HeapAlloc;

#[alloc_error_handler]
fn alloc(_: Layout) -> ! { loop {} }

fn print_sp(i: i32) {
    unsafe {
        printf(b"%d \0" as *const u8, i);
    }
}

struct Counter {
    count: usize,
}

impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let _x: Box<u8> = Box::new(4);
    let mut ctr = Counter::new();
    print_sp(ctr.next().unwrap() as i32);
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
