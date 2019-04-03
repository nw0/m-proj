#![feature(lang_items)]
#![feature(optin_builtin_traits)]
#![feature(on_unimplemented)]
#![feature(alloc)]
#![feature(alloc_error_handler)]

#![no_std]

extern crate alloc;

mod support;

use alloc::boxed::Box;
use support::{printf,HeapAlloc};

#[lang = "start"]
fn start(_main: *const u8, _argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}

#[global_allocator]
static HEAP: HeapAlloc = HeapAlloc;

fn print_sp(i: i32) {
    unsafe {
        printf(b"%d \0" as *const u8, i);
    }
}

fn main() {
    let x: Box<i32> = Box::new(4);
    print_sp(*x);
}
