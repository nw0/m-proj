#![feature(no_core)]
#![feature(lang_items)]
#![feature(optin_builtin_traits)]
#![feature(on_unimplemented)]

#![no_core]

extern crate libcore_cheri;

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

fn fact(x: u32) -> u32 {
    match x {
        0 => 1,
        _ => x * fact(x - 1)
    }
}

fn main() {
    let mut i: i32 = 1;
    i *= 2;
    let j: i32 = i;

    unsafe {
        printf(b"Hello, world, %d\n\0" as *const u8, fact(4));
        printf(b"8 mod 3 = %d\n\0" as *const u8, 8 % 3);
        printf(b"10 / 3 = %d\n\0" as *const u8, 10u32 / 3 as u32);
        printf(b"A negative number: %d\n\0" as *const u8, -j);
    }
}
