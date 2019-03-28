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

fn main() {
    let a1: [i32; 1] = [0];
    let a2: [i32; 1] = [0];
    let a3 = a1.iter().zip(a2.iter());
}
