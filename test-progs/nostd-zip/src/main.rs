#![feature(no_core)]
#![feature(lang_items)]
#![feature(optin_builtin_traits)]
#![feature(on_unimplemented)]

#![no_std]

use core::mem;
use core::iter::Iterator;

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
    a1.iter().for_each(|x| print_sp(*x));

    unsafe { printf(b"\n\0" as *const u8); }
    unsafe { printf(b"len of array above: %d\n\0" as *const u8, a1.len() as u32); }
    unsafe { printf(b"len of `hello`: %d\n\0" as *const u8, "hello".chars().count() as u32); }
    unsafe { printf(b"usize width: %d bytes\n\0" as *const u8, mem::size_of::<usize>() as u32); }
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
