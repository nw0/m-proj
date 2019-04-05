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
    unsafe { printf(b"Recall Vec::new() has capacity 0\n\0" as *const u8); }

    unsafe { printf(b"A vector with cap 2: \0" as *const u8); }
    let mut v: Vec<i32> = Vec::with_capacity(2);
    v.push(14);
    v.push(11);
    for e in v.iter() {
        print_sp(*e);
    }

    unsafe { printf(b"\nPushing a third int: \0" as *const u8); }
    v.push(12);
    for e in v.iter() {
        print_sp(*e);
    }

    unsafe { printf(b"\nindexing in 0, 1, 2: \0" as *const u8); }
    for i in 0..3 {
        print_sp(v[i]);
    }

    unsafe { printf(b"\nsorting the vector: \0" as *const u8); }
    v.sort();
    for e in v.iter() {
        print_sp(*e);
    }

    unsafe { printf(b"\npop after extension: \0" as *const u8); }
    v.pop();
    for e in v.iter() {
        print_sp(*e);
    }

    unsafe { printf(b"\nanother vector: \0" as *const u8); }
    use alloc::vec;
    let u = vec![0, 3, -2, -1, 8];
    u.iter().for_each(|x| print_sp(*x));
    unsafe { printf(b"\n\0" as *const u8); }
}
