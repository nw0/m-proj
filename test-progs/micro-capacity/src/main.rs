#![feature(lang_items)]
#![feature(optin_builtin_traits)]
#![feature(on_unimplemented)]
#![feature(alloc)]
#![feature(alloc_error_handler)]

#![no_std]

extern crate alloc;

mod support;

use support::{printf,HeapAlloc};

#[lang = "start"]
fn start(_main: *const u8, _argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}

#[global_allocator]
static HEAP: HeapAlloc = HeapAlloc;

fn print_sp(i: i64) {
    unsafe {
        printf(b"%d.\0" as *const u8, i);
    }
}

fn main() {
    use alloc::collections::VecDeque;
    let mut deque = VecDeque::with_capacity(31);
    deque.push_front(5);
    for x in &deque {
        print_sp(*x);
    }
    unsafe { printf(b"\n\0" as *const u8); }
    print_sp(deque.head as i64);
    print_sp(deque.tail as i64);
    unsafe { printf(b"\n\0" as *const u8); }
    deque.reserve(30);
    print_sp(deque.head as i64);
    print_sp(deque.tail as i64);
    unsafe { printf(b"\n\0" as *const u8); }
    deque.push_back(6);
    for x in &deque {
        print_sp(*x);
    }
    unsafe { printf(b"\n\0" as *const u8); }
    print_sp(deque.head as i64);
    print_sp(deque.tail as i64);
    unsafe { printf(b"\n\0" as *const u8); }
}
