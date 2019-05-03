#![feature(lang_items)]
#![feature(optin_builtin_traits)]
#![feature(on_unimplemented)]
#![feature(alloc)]
#![feature(alloc_error_handler)]

#![no_std]

extern crate alloc;

#[macro_use]
mod support;

use alloc::borrow::ToOwned;
use alloc::vec::Vec;
use support::{printf,HeapAlloc};

#[lang = "start"]
fn start(_main: *const u8, _argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}

#[global_allocator]
static HEAP: HeapAlloc = HeapAlloc;

fn main() {
    use core::ptr::copy_nonoverlapping;

    let s: [i64; 3] = [1, 2 ,3];
    let reps = 6148914691236517207;             // (2 ** 64 + 5) / 3

    let mut buf: Vec<i64> = Vec::with_capacity((s.len() * reps) as u64 as usize);
    prntf!("capacity: %d\n", buf.capacity() as u64);
    buf.extend(&s);

    let mut overwritten: Vec<i64> = Vec::new();
    overwritten.extend(&s);
    overwritten[0] = -1;
    overwritten.push(-4);
    for x in &overwritten { prntf!("%d.", *x); } prntf!("\n");
    // prntf!("%p %p\n", &(buf[0]), &(u[0]));

    {
        let mut m = 8;  // 8 <= n >> 1
        while m > 0 {
            unsafe {
                copy_nonoverlapping(
                    buf.as_ptr(),
                    (buf.as_mut_ptr() as *mut i64).add(buf.len()),
                    buf.len()
                );
                let buf_len = buf.len();
                buf.set_len(buf_len * 2);
            }
            m >>= 1;
        }
    }
    // prntf!("%d", buf.len() as i64);
    for x in &overwritten { prntf!("%d.", *x); } prntf!("\n");
}
