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

fn print_sp(i: i64) {
    unsafe {
        printf(b"%d.\0" as *const u8, i);
    }
}

fn main() {
    let s = [1, 2, 3 as i64];
    // rep * 3 == 2 ** 64 + 5
    let rep = 6148914691236517207;

    use alloc::boxed::Box;
    use alloc::vec;
    use core::ptr::{copy_nonoverlapping, swap};

    unsafe { printf(b"create v capacity: %d\n\0" as *const u8, (s.len() * rep) as u64 as usize as u32); }
    let mut v: Vec<i64> = Vec::with_capacity((s.len() * rep) as u64 as usize);
    unsafe { printf(b"capacity of v: %d\n\0" as *const u8, v.capacity() as u32); }
    v.extend(&s);
    for x in &v {
        print_sp(*x);
    }
    unsafe { printf(b"\n\0" as *const u8); }
    let mut u: Vec<i64> = Vec::new();
    u.extend(&s);
    u[0] = 7;
    u.push(0);
    unsafe { printf(b"%p %p\n\0" as *const u8, &(v[0]), &(u[0])); }

    unsafe {
        let m = 512;
        copy_nonoverlapping(v.as_ptr(), (v.as_mut_ptr() as *mut i64).add(3), 3);
        v.set_len(6);
        copy_nonoverlapping(v.as_ptr(), (v.as_mut_ptr() as *mut i64).add(6), 6);
        v.set_len(12);
        copy_nonoverlapping(v.as_ptr(), (v.as_mut_ptr() as *mut i64).add(12), 12);
        v.set_len(24);
    }
    print_sp(v.len() as i64);
    for x in &v {
        print_sp(*x);
    }
    unsafe { printf(b"\n\0" as *const u8); }
    for x in &u {
        print_sp(*x);
    }
    unsafe { printf(b"\n\0" as *const u8); }
}
