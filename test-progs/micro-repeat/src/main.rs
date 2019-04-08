#![feature(lang_items)]
#![feature(optin_builtin_traits)]
#![feature(on_unimplemented)]
#![feature(alloc)]
#![feature(alloc_error_handler)]

#![no_std]

extern crate alloc;

mod support;

use alloc::vec::Vec;
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
        printf(b"%d.\0" as *const u8, i);
    }
}

fn main() {
    let s = [0, 1, 2 as i32];
    // rep * 3 == 2 ** 64 + 5
    let rep = 6148914691236517207;
    let mut v: Vec<i32> = Vec::with_capacity(s.len() * rep);
    let mut over: i32 = 8;
    v.extend(&s);

    use core::ptr::copy_nonoverlapping;
    unsafe {
        copy_nonoverlapping(v.as_ptr(), (v.as_mut_ptr() as *mut i32).add(3), 3);
        v.set_len(6);
    }
    print_sp(over);
}
