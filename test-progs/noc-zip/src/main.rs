#![feature(no_core)]
#![feature(lang_items)]
#![feature(optin_builtin_traits)]
#![feature(on_unimplemented)]

#![no_core]

extern crate libcore_cheri as core;

use core::prelude::Iterator;

#[cfg(target_os = "freebsd")]
#[link(name = "c")]
extern {
    pub fn printf(format: *const u8, ...) -> i32;
}

#[lang = "start"]
fn start(_main: *const u8, _argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}

fn print_sp(i: i32) {
    unsafe {
        printf(b"%d \0" as *const u8, i);
    }
}

fn main() {
    let a1: [i32; 4] = [1, 3, 4, 5];
    let a2: [i32; 4] = [3, 3, 3, 4];
    a1.iter().zip(a2.iter()).for_each(|(a, b)| print_sp(*a + *b));
    unsafe { printf(b"\n\0" as *const u8); }
}
